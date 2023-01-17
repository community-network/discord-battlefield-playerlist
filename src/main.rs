use serenity::{
    client::{Client, Context, EventHandler},
    model::{gateway::Ready, prelude::ChannelId},
    prelude::GatewayIntents,
};
use std::{
    sync::{atomic, Arc},
    time,
};
use structs::{config::SeederConfig, player_list::PlayerList};
mod structs;
mod to_table;

use warp::Filter;

struct Handler;

#[serenity::async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, _: Ready) {
        let user = ctx.cache.current_user();
        log::info!("Logged in as {:#?}", user.name);

        let mut cfg = get_config().await;
        confy::store_path("config.txt", cfg.clone()).unwrap();

        let last_update = Arc::new(atomic::AtomicI64::new(0));
        let last_update_clone = Arc::clone(&last_update);

        // healthcheck
        tokio::spawn(async move {
            let hello = warp::any().map(move || {
                let last_update_i64 = last_update_clone.load(atomic::Ordering::Relaxed);
                let now_minutes = chrono::Utc::now().timestamp() / 60;
                if (now_minutes - last_update_i64) > 5 {
                    warp::reply::with_status(
                        format!("{}", now_minutes - last_update_i64),
                        warp::http::StatusCode::SERVICE_UNAVAILABLE,
                    )
                } else {
                    warp::reply::with_status(
                        format!("{}", now_minutes - last_update_i64),
                        warp::http::StatusCode::OK,
                    )
                }
            });
            warp::serve(hello).run(([0, 0, 0, 0], 3030)).await;
        });

        let client = reqwest::Client::new();
        // loop in seperate async
        tokio::spawn(async move {
            loop {
                cfg = match gather_table(&cfg.server_name, &client).await {
                    Ok((_player_list, title, tables)) => send_info(&ctx, cfg, tables, title).await,
                    Err(e) => {
                        log::error!("Couldn't get serverinfo: {:#?}", e);
                        cfg
                    }
                };

                // wait 30 seconds before redo
                tokio::time::sleep(time::Duration::from_secs(30)).await;
            }
        });
    }
}

async fn send_info(
    ctx: &Context,
    mut cfg: SeederConfig,
    tables: Vec<String>,
    title: String,
) -> SeederConfig {
    if cfg.messages.len() >= 5 {
        let mut index = 0;

        ChannelId(cfg.channel)
            .edit_message(&ctx.http, cfg.messages[index], |m| {
                m.content(format!("```\n{}\n```", title))
            })
            .await
            .unwrap();

        index += 1;

        for content in tables.iter() {
            let text: Vec<&str> = content.split("\n").collect();
            let (first_message, second_message) = text.split_at(text.len() / 2);

            ChannelId(cfg.channel)
                .edit_message(&ctx.http, cfg.messages[index], |m| {
                    m.content(format!("```\n{}\n```", first_message.join("\n")))
                })
                .await
                .unwrap();

            index += 1;

            ChannelId(cfg.channel)
                .edit_message(&ctx.http, cfg.messages[index], |m| {
                    m.content(format!("```\n{}\n```", second_message.join("\n")))
                })
                .await
                .unwrap();

            index += 1;
        }
    } else {
        log::info!("Message to edit not set, creating new...");

        cfg.messages = vec![];

        let title_result = ChannelId(cfg.channel)
            .send_message(&ctx.http, |m| m.content(format!("```\n{}\n```", title)))
            .await
            .unwrap();
        cfg.messages.push(*title_result.id.as_u64());

        for content in tables.iter() {
            let text: Vec<&str> = content.split("\n").collect();
            let (first_message, second_message) = text.split_at(text.len() / 2);

            let first_result = ChannelId(cfg.channel)
                .send_message(&ctx.http, |m| {
                    m.content(format!("```\n{}\n```", first_message.join("\n")))
                })
                .await
                .unwrap();
            cfg.messages.push(*first_result.id.as_u64());

            let second_result = ChannelId(cfg.channel)
                .send_message(&ctx.http, |m| {
                    m.content(format!("```\n{}\n```", second_message.join("\n")))
                })
                .await
                .unwrap();
            cfg.messages.push(*second_result.id.as_u64());
        }
        confy::store_path("config.txt", cfg.clone()).unwrap();
    }
    cfg
}

async fn gather_table(
    server: &str,
    client: &reqwest::Client,
) -> anyhow::Result<(PlayerList, String, Vec<String>)> {
    let result = to_table::player_list::request_player_list(server, client)
        .await
        .unwrap();

    let (title, tables) =
        match to_table::seeder_player_list::request_player_list(server, client).await {
            Ok(seeder_result) => {
                to_table::seeder_player_list::to_tables(&seeder_result, &result.clone()).await
            }
            Err(_) => to_table::player_list::to_tables(&result).await,
        };

    Ok((result, title, tables))
}

async fn get_config() -> structs::config::SeederConfig {
    let cfg: structs::config::SeederConfig = match confy::load_path("config.txt") {
        Ok(config) => config,
        Err(e) => {
            log::error!("error in config.txt: {}", e);
            log::warn!("changing back to default..");
            structs::config::SeederConfig {
                server_name: "".into(),
                token: "".into(),
                channel: 0,
                messages: vec![],
            }
        }
    };
    if cfg.server_name.is_empty() {
        log::error!("servername isn't set!");
    }
    if cfg.token.is_empty() {
        log::error!("token isn't set!");
    }
    if cfg.token.is_empty() {
        log::error!("channel isn't set!");
    }
    cfg
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    flexi_logger::Logger::try_with_str("warn,discord_playerlist=info")
        .unwrap_or_else(|e| panic!("Logger initialization failed with {}", e))
        .start()?;

    let cfg = get_config().await;

    // Login with a bot token from the environment
    let intents = GatewayIntents::non_privileged();
    let mut client = Client::builder(cfg.token, intents)
        .event_handler(Handler)
        .await
        .expect("Error creating client");

    // start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        log::error!("Client error: {:?}", why);
    }
    Ok(())
}
