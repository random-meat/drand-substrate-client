use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Info {
    pub public_key: Value,
    pub period: u64,
    pub genesis_time: u64,
    pub hash: Value,
    #[serde(rename = "groupHash")]
    pub group_hash: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Round {
    pub round: u32,
    pub randomness: Value,
    pub previous_signature: Value,
    pub signature: Value,
}
