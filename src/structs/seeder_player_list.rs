use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SeederServerInfo {
    pub country: String,
    pub description: String,
    pub level: String,
    pub maps: Option<Vec<String>>,
    pub mode: String,
    pub name: String,
    pub owner: String,
    pub region: String,
    pub servertype: String,
    pub settings: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SeederGameItem {
    pub id: Option<String>,
    pub name: Option<String>,
    #[serde(rename = "shortName")]
    pub short_name: Option<String>,
    pub image: Option<String>,
    #[serde(rename = "type")]
    pub _type: Option<String>,
    pub subtype: Option<String>,
    pub class: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SeederIngameChat {
    pub timestamp: String,
    pub sender: String,
    pub content: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SeederPlayerPlatoon {
    pub tag: String,
    pub name: String,
    pub icon: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ClassIcons {
    pub id: Option<String>,
    pub black: Option<String>,
    pub white: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SeederPlayerClass {
    pub class_id: String,
    pub class_name: Option<String>,
    pub class_kit: Option<String>,
    pub class_info1: Option<String>,
    pub class_info2: Option<String>,
    pub class_icons: Option<ClassIcons>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SeederServerPlayer {
    pub index: u64,
    #[serde(rename = "teamId")]
    pub team_id: u64,
    pub mark: u64,
    pub platoon: SeederPlayerPlatoon,
    pub squad_id: u64,
    pub squad_name: String,
    pub rank: u64,
    pub name: String,
    pub player_id: u64,
    pub kills: u64,
    pub deaths: u64,
    pub score: u64,
    pub player_class: SeederPlayerClass,
    #[serde(rename = "Spectator")]
    pub spectator: u64,
    pub vehicle: SeederGameItem,
    pub weapons: Vec<SeederGameItem>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SeederServerTeam {
    pub teamid: String,
    pub players: Vec<SeederServerPlayer>,
    pub image: String,
    pub key: String,
    pub name: String,
    pub faction: String,
    pub score: i64,
    #[serde(rename = "scoreFromKills")]
    pub score_from_kills: i64,
    #[serde(rename = "scoreFromFlags")]
    pub score_from_flags: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SeederPlayerList {
    pub _id: String,
    #[serde(rename = "gameId")]
    pub game_id: i64,
    #[serde(rename = "ingameChat")]
    pub ingame_chat: Vec<SeederIngameChat>,
    pub serverinfo: SeederServerInfo,
    pub teams: Vec<SeederServerTeam>,
    #[serde(rename = "timeStamp")]
    pub time_stamp: String,
    pub update_timestamp: i64,
}
