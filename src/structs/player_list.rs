use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ServerInfo {
    pub name: String,
    pub description: String,
    pub region: String,
    pub country: String,
    pub level: String,
    pub mode: String,
    pub maps: Option<Vec<String>>,
    pub owner: String,
    pub settings: Option<Vec<String>>,
    pub servertype: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GamePlayer {
    pub join_time: i64,
    pub latency: u64,
    pub name: String,
    pub platoon: String,
    pub player_id: u64,
    pub rank: u64,
    pub slot: u64,
    pub user_id: u64,
    pub platform: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GameTeam {
    pub teamid: String,
    pub players: Vec<GamePlayer>,
    pub image: String,
    pub key: String,
    pub name: String,
    pub faction: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PlayerList {
    pub serverinfo: ServerInfo,
    pub teams: Vec<GameTeam>,
    pub que: Vec<GamePlayer>,
    pub loading: Vec<GamePlayer>,
    pub update_timestamp: i64,
}
