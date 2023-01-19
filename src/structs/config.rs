use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct SenderConfig {
    pub server_name: String,
    pub game: Games,
    pub token: String,
    pub channel: u64,
    pub messages: Vec<u64>,
}

/// `SeederConfig` implements `Default`
impl ::std::default::Default for SenderConfig {
    fn default() -> Self {
        Self {
            server_name: "".into(),
            token: "".into(),
            channel: 0,
            messages: vec![],
            game: Games::from("bf1"),
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub enum Games {
    Bf4,
    Bf1,
}

impl Games {
    pub fn from(input: &str) -> Games {
        match input {
            "bf4" => Games::Bf4,
            "bf1" => Games::Bf1,
            _ => Games::Bf1,
        }
    }
}
