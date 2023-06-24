use anyhow::Result;
use reqwest::Url;
use tabular::row;

use crate::structs;

pub async fn request_player_list(
    server_name: &str,
    client: &reqwest::Client,
) -> Result<structs::bf4_player_list::DetailedServerInfo> {
    let mut url = Url::parse("http://api.gametools.network/bf4/detailedserver/").unwrap();
    url.query_pairs_mut().append_pair("name", server_name);

    Ok(client
        .get(url)
        .send()
        .await?
        .json::<structs::bf4_player_list::DetailedServerInfo>()
        .await?)
}

pub async fn to_tables(
    result: &structs::bf4_player_list::DetailedServerInfo,
) -> (String, Vec<String>) {
    let mut teams_result: Vec<String> = vec![];
    match &result.teams {
        Some(teams) => {
            for team in teams {
                let mut table = tabular::Table::new("{:<}  {:<}  {:<}  {:<}")
                    .with_heading(team.teamid.clone())
                    .with_row(row!("Rank", "Name", "Score", "KD",));

                if team.players.len() <= 0 {
                    table.add_row(row!("N/A", "This team is empty", "N/A", "N/A"));
                }

                for player in &team.players {
                    let mut player_name = player.name.clone();
                    if !player.tag.is_empty() {
                        player_name = format!("[{}]{}", player.tag.clone(), player_name);
                    }

                    table.add_row(row!(
                        player.rank,
                        player_name,
                        player.score,
                        format!("{}/{}", player.kills, player.deaths),
                    ));
                }

                teams_result.push(table.to_string());
            }

            let title = title_table(result).await;

            (title, teams_result)
        }
        None => todo!(),
    }
}

async fn title_table(server_info: &structs::bf4_player_list::DetailedServerInfo) -> String {
    let table = tabular::Table::new("{:<}  {:<}  {:<}")
        .with_heading(format!("{} playerlist", server_info.prefix))
        .with_row(row!("Last bot update", "Current map", "Favorites"))
        .with_row(row!(
            chrono::Utc::now().format("%T %b %e %Y"),
            format!("{} - {}", server_info.current_map, server_info.mode),
            &server_info.favorites,
        ));

    table.to_string()
}
