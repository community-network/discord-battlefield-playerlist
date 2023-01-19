use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ScoreServerPlayer {
    pub kills: i64,
    pub deaths: i64,
    pub name: String,
    pub player_id: String,
    pub rank: i64,
    pub role: i64,
    pub score: i64,
    pub squad: i64,
    pub tag: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ScoreTeamList {
    pub players: Vec<ScoreServerPlayer>,
    pub teamid: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DetailedServerInfo {
    pub prefix: String,
    #[serde(rename = "currentMap")]
    pub current_map: String,
    pub mode: String,
    pub favorites: String,
    pub teams: Option<Vec<ScoreTeamList>>,
    pub players: Option<Vec<ScoreServerPlayer>>,
}
