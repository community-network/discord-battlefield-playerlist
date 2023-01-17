use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct SeederConfig {
    pub server_name: String,
    pub token: String,
    pub channel: u64,
    pub messages: Vec<u64>,
}

/// `SeederConfig` implements `Default`
impl ::std::default::Default for SeederConfig {
    fn default() -> Self {
        Self {
            server_name: "".into(),
            token: "".into(),
            channel: 0,
            messages: vec![],
        }
    }
}
