use std::collections::HashMap;

use anyhow::Result;
use chrono::Utc;
use reqwest::Url;
use tabular::row;

use crate::structs;

pub async fn request_player_list(
    server_name: &str,
    client: &reqwest::Client,
) -> Result<structs::seeder_player_list::SeederPlayerList> {
    let mut url = Url::parse("https://api.gametools.network/bf1/seederplayers/").unwrap();
    url.query_pairs_mut().append_pair("name", server_name);
    let result = client
        .get(url)
        .send()
        .await?
        .json::<structs::seeder_player_list::SeederPlayerList>()
        .await?;

    let timestamp = chrono::DateTime::<Utc>::from_utc(
        match chrono::NaiveDateTime::from_timestamp_opt(result.update_timestamp, 0) {
            Some(naive_time) => naive_time,
            None => chrono::NaiveDateTime::default(),
        },
        Utc,
    );
    let current = chrono::Utc::now();

    // use only main playerlist if data is to old
    if (current - timestamp).num_seconds() > 30 {
        anyhow::bail!("Playerlist to old to be usable")
    }
    Ok(result)
}

pub async fn to_tables(
    seeder_result: &structs::seeder_player_list::SeederPlayerList,
    result: &structs::player_list::PlayerList,
) -> (String, Vec<String>) {
    let mut players = HashMap::new();
    for team in &result.teams {
        for player in &team.players {
            players.insert(player.player_id, player);
        }
    }

    let mut teams: Vec<String> = vec![];
    for team in &seeder_result.teams {
        let mut table = tabular::Table::new("{:<}  {:<}  {:<}  {:<}  {:<}  {:<}  {:<}  {:<}")
            .with_heading(format!("{} - score: {}", team.name, team.score))
            .with_row(row!(
                "Squad", "Class", "Rank", "Name", "Score", "KD", "Ping", "Playtime"
            ));

        let mut sorted_players = team.players.clone();
        sorted_players.sort_by_key(|item| item.index);

        if sorted_players.len() <= 0 {
            table.add_row(row!(
                "N/A",
                "This team is empty",
                "N/A",
                "N/A",
                "N/A",
                "N/A",
                "N/A",
                "N/A"
            ));
        }

        for seeder_player in sorted_players {
            let player = players.get(&seeder_player.player_id);

            let mut player_name = seeder_player.name;
            if !seeder_player.platoon.name.is_empty() {
                player_name = format!("[{}]{}", seeder_player.platoon.name, player_name);
            }

            let player_join_time = match player {
                Some(player) => {
                    let mut f = timeago::Formatter::new();
                    f.ago("");
                    let timestamp = chrono::DateTime::<Utc>::from_utc(
                        match chrono::NaiveDateTime::from_timestamp_opt(
                            player.join_time / 1000000,
                            0,
                        ) {
                            Some(naive_time) => naive_time,
                            None => chrono::NaiveDateTime::default(),
                        },
                        Utc,
                    );
                    let current = chrono::Utc::now();

                    f.convert_chrono(timestamp, current)
                }
                None => "".into(),
            };

            let player_latency = match player {
                Some(player) => format!("{}ms", player.latency),
                None => "?".into(),
            };

            table.add_row(row!(
                seeder_player.squad_name,
                match seeder_player.player_class.class_name {
                    Some(class) => class,
                    None => "".into(),
                },
                seeder_player.rank,
                player_name,
                seeder_player.score,
                format!("{}/{}", seeder_player.kills, seeder_player.deaths),
                player_latency,
                player_join_time
            ));
        }

        teams.push(table.to_string());
    }

    let title = super::title_table(result, Some(seeder_result)).await;

    (title, teams)
}
