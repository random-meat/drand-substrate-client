use codec::{Decode, Encode};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sp_runtime::{traits::ConstU32, BoundedVec, RuntimeDebug};

use crate::util::str_to_bounded_vec;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InfoRaw {
    pub public_key: Value,
    pub period: u64,
    pub genesis_time: u64,
    pub hash: Value,
    #[serde(rename = "groupHash")]
    pub group_hash: Value,
}

#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, scale_info::TypeInfo)]
pub struct Info {
    pub public_key: BoundedVec<u8, ConstU32<48>>,
    pub period: u64,
    pub genesis_time: u64,
    pub hash: BoundedVec<u8, ConstU32<32>>,
    pub group_hash: BoundedVec<u8, ConstU32<32>>,
}

impl From<InfoRaw> for Info {
    fn from(info: InfoRaw) -> Self {
        Info {
            public_key: str_to_bounded_vec::<48>(&info.public_key),
            period: info.period,
            genesis_time: info.genesis_time,
            hash: str_to_bounded_vec::<32>(&info.hash),
            group_hash: str_to_bounded_vec::<32>(&info.group_hash),
        }
    }
}

/// RoundRaw is used for http interactions and JSON parsing with serde.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RoundRaw {
    pub round: u32,
    pub randomness: Value,
    pub previous_signature: Value,
    pub signature: Value,
}

/// Round will be used in substrate, as it has the Encode/Decode traits derived.
#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, scale_info::TypeInfo)]
pub struct Round {
    pub round: u32,
    pub randomness: BoundedVec<u8, ConstU32<32>>,
    pub previous_signature: BoundedVec<u8, ConstU32<96>>,
    pub signature: BoundedVec<u8, ConstU32<96>>,
}

impl From<RoundRaw> for Round {
    fn from(round: RoundRaw) -> Self {
        Round {
            round: round.round,
            randomness: str_to_bounded_vec::<32>(&round.randomness),
            previous_signature: str_to_bounded_vec::<96>(&round.previous_signature),
            signature: str_to_bounded_vec::<96>(&round.signature),
        }
    }
}
