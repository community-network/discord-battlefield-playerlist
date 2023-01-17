use anyhow::Result;
use chrono::Utc;
use reqwest::Url;
use tabular::row;

use crate::structs;

pub async fn request_player_list(
    server_name: &str,
    client: &reqwest::Client,
) -> Result<structs::player_list::PlayerList> {
    let mut url = Url::parse("https://api.gametools.network/bf1/players/").unwrap();
    url.query_pairs_mut().append_pair("name", server_name);

    Ok(client
        .get(url)
        .send()
        .await?
        .json::<structs::player_list::PlayerList>()
        .await?)
}

pub async fn to_tables(result: &structs::player_list::PlayerList) -> (String, Vec<String>) {
    let mut teams: Vec<String> = vec![];
    for team in &result.teams {
        let mut table = tabular::Table::new("{:<}  {:<}  {:<}  {:<}")
            .with_heading(team.name.clone())
            .with_row(row!("Rank", "Name", "Ping", "Playtime"));

        let mut sorted_players = team.players.clone();
        sorted_players.sort_by_key(|item| item.slot);

        if sorted_players.len() <= 0 {
            table.add_row(row!("N/A", "This team is empty", "N/A", "N/A"));
        }

        for player in sorted_players {
            let mut player_name = player.name;
            if !player.platoon.is_empty() {
                player_name = format!("[{}]{}", player.platoon, player_name);
            }

            let mut f = timeago::Formatter::new();
            f.ago("");
            let timestamp = chrono::DateTime::<Utc>::from_utc(
                match chrono::NaiveDateTime::from_timestamp_opt(player.join_time / 1000000, 0) {
                    Some(naive_time) => naive_time,
                    None => chrono::NaiveDateTime::default(),
                },
                Utc,
            );
            let current = chrono::Utc::now();

            table.add_row(row!(
                player.rank,
                player_name,
                format!("{}ms", player.latency),
                f.convert_chrono(timestamp, current)
            ));
        }

        teams.push(table.to_string());
    }

    let title = super::title_table(result, None).await;

    (title, teams)
}
