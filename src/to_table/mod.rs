use crate::structs;

use chrono::Utc;
use tabular::row;
pub mod player_list;
pub mod seeder_player_list;

async fn title_table(
    result: &structs::player_list::PlayerList,
    seeder_player_list: Option<&structs::seeder_player_list::SeederPlayerList>,
) -> String {
    let timestamp = chrono::DateTime::<Utc>::from_utc(
        match chrono::NaiveDateTime::from_timestamp_opt(result.update_timestamp, 0) {
            Some(naive_time) => naive_time,
            None => chrono::NaiveDateTime::default(),
        },
        Utc,
    );

    let seeder_time = match seeder_player_list {
        Some(seeder_info) => chrono::DateTime::<Utc>::from_utc(
            match chrono::NaiveDateTime::from_timestamp_opt(seeder_info.update_timestamp, 0) {
                Some(naive_time) => naive_time,
                None => chrono::NaiveDateTime::default(),
            },
            Utc,
        )
        .format("%T %b %e %Y")
        .to_string(),
        None => "Not running".into(),
    };

    let table = tabular::Table::new("{:<}  {:<}  {:<}")
        .with_heading(format!("{} playerlist", result.serverinfo.name))
        .with_row(row!("Last bot update", "Gametools update", "Gather update"))
        .with_row(row!(
            chrono::Utc::now().format("%T %b %e %Y"),
            timestamp.format("%T %b %e %Y"),
            seeder_time
        ));

    table.to_string()
}
