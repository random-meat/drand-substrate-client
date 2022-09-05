use serde::{Serialize, Deserialize};
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Info {
    pub public_key: Value,
    pub period: u64,
    pub genesis_time: u64,
    pub hash: Value,
    #[serde(rename = "groupHash")]
    pub group_hash: Value,
    #[serde(rename = "schemeID")]
    pub scheme_id: Value,
    pub metadata: Metadata,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Metadata {
    #[serde(rename = "beaconID")]
    pub beacon_id: Value,
}
