use serenity::{
    client::{Client, Context, EventHandler},
    model::{gateway::Ready, prelude::ChannelId},
    prelude::GatewayIntents,
};
use std::{
    sync::{atomic, Arc},
    time, vec,
};
use warp::Filter;
mod structs;
mod to_table;

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
                cfg = match gather_table(&cfg, &client).await {
                    Ok((title, tables)) => {
                        match send_info(&ctx, cfg.clone(), tables, title).await {
                            Ok(cfg) => cfg,
                            Err(e) => {
                                log::error!("Couldn't send message: {:#?}", e);
                                cfg
                            }
                        }
                    }
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
    mut cfg: structs::config::SenderConfig,
    tables: Vec<String>,
    title: String,
) -> anyhow::Result<structs::config::SenderConfig> {
    if cfg.messages.len() >= 5 {
        let mut index = 0;

        match ChannelId(cfg.channel)
            .edit_message(&ctx.http, cfg.messages[index], |m| {
                m.content(format!("```\n{}\n```", title))
            })
            .await
        {
            Ok(_) => {}
            Err(e) => log::error!("Failed to send title message: {:#?}", e),
        };

        index += 1;

        for content in tables.iter() {
            let text: Vec<&str> = content.split("\n").collect();
            let (first_message, second_message) = text.split_at(text.len() / 2);

            match ChannelId(cfg.channel)
                .edit_message(&ctx.http, cfg.messages[index], |m| {
                    m.content(format!("```\n{}\n```", first_message.join("\n")))
                })
                .await
            {
                Ok(_) => {}
                Err(e) => log::error!("Failed to send first team's message: {:#?}", e),
            };

            index += 1;

            match ChannelId(cfg.channel)
                .edit_message(&ctx.http, cfg.messages[index], |m| {
                    m.content(format!("```\n{}\n```", second_message.join("\n")))
                })
                .await
            {
                Ok(_) => {}
                Err(e) => log::error!("Failed to send second team's message: {:#?}", e),
            };

            index += 1;
        }
    } else {
        log::info!("Message to edit not set, creating new...");

        cfg.messages = vec![];

        match ChannelId(cfg.channel)
            .send_message(&ctx.http, |m| m.content(format!("```\n{}\n```", title)))
            .await
        {
            Ok(title_result) => {
                cfg.messages.push(*title_result.id.as_u64());
            }
            Err(e) => {
                cfg.messages.clear();
                anyhow::bail!("Failed to set message: {}", e);
            }
        };

        for content in tables.iter() {
            let text: Vec<&str> = content.split("\n").collect();
            let (first_message, second_message) = text.split_at(text.len() / 2);

            match ChannelId(cfg.channel)
                .send_message(&ctx.http, |m| {
                    m.content(format!("```\n{}\n```", first_message.join("\n")))
                })
                .await
            {
                Ok(first_result) => {
                    cfg.messages.push(*first_result.id.as_u64());
                }
                Err(e) => {
                    cfg.messages.clear();
                    anyhow::bail!("Failed to set message: {}", e);
                }
            };

            match ChannelId(cfg.channel)
                .send_message(&ctx.http, |m| {
                    m.content(format!("```\n{}\n```", second_message.join("\n")))
                })
                .await
            {
                Ok(second_result) => {
                    cfg.messages.push(*second_result.id.as_u64());
                }
                Err(e) => {
                    cfg.messages.clear();
                    anyhow::bail!("Failed to set message: {}", e);
                }
            };
        }
        confy::store_path("config.txt", cfg.clone()).unwrap();
    }
    Ok(cfg)
}

async fn gather_table(
    cfg: &structs::config::SenderConfig,
    client: &reqwest::Client,
) -> anyhow::Result<(String, Vec<String>)> {
    Ok(match cfg.game {
        structs::config::Games::Bf1 => {
            let result = match to_table::player_list::request_player_list(&cfg.server_name, client)
                .await
            {
                Ok(result) => result,
                // retry
                Err(_) => {
                    match to_table::player_list::request_player_list(&cfg.server_name, client).await
                    {
                        Ok(result) => result,
                        Err(e) => anyhow::bail!("Couldn't get bf1 playerlist {:#?}", e),
                    }
                }
            };

            let (title, tables) =
                match to_table::seeder_player_list::request_player_list(&cfg.server_name, client)
                    .await
                {
                    Ok(seeder_result) => {
                        to_table::seeder_player_list::to_tables(&seeder_result, &result.clone())
                            .await
                    }
                    Err(_) => to_table::player_list::to_tables(&result).await,
                };

            (title, tables)
        }
        structs::config::Games::Bf4 => {
            let result = match to_table::bf4_player_list::request_player_list(
                &cfg.server_name,
                client,
            )
            .await
            {
                Ok(result) => result,
                Err(e) => anyhow::bail!("Couldn't get bf4 playerlist {:#?}", e),
            };

            to_table::bf4_player_list::to_tables(&result).await
        }
    })
}

async fn get_config() -> structs::config::SenderConfig {
    let cfg: structs::config::SenderConfig = match confy::load_path("config.txt") {
        Ok(config) => config,
        Err(e) => {
            log::error!("error in config.txt: {}", e);
            log::warn!("changing back to default..");
            structs::config::SenderConfig {
                server_name: "".into(),
                token: "".into(),
                channel: 0,
                messages: vec![],
                game: structs::config::Games::from("bf1"),
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
