#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Info {
    pub public_key: String,
    pub period: u64,
    pub genesis_time: u64,
    pub hash: String,
    #[serde(rename = "groupHash")]
    pub group_hash: String,
    #[serde(rename = "schemeID")]
    pub scheme_id: String,
    pub metadata: Metadata,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Metadata {
    #[serde(rename = "beaconID")] 
    pub beacon_id: String,
}
