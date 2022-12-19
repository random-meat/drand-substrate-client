//! This file contains the data structs returned by the drand API.
//! There are two types of structs for each endpoint:
//! 1. The raw struct (e.g. `InfoRaw`), which can be deserialized from the JSON response directly.
//! While the raw struct can be directly imported into substrate, its fields cannot
//! be used as storage item because they don't derive `parity_scale_codec`.
//! 2. The SCALE-encodeable struct (e.g. `Info`), that can be saved to the chain storage.
//! The SCALE-encodeable struct `T` can be directly constructed from the raw struct, since it
//! implements `TryFrom<RawT>` trait.

use codec::{Decode, Encode};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sp_runtime::{traits::ConstU32, BoundedVec, RuntimeDebug};

use crate::util::hex_json_value_to_bounded_vec_u8;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChainsRaw {
    pub hash: Value,
}

/// This is should be returned from the `/chains` endpoint of a node.
#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, scale_info::TypeInfo)]
pub struct Chains {
    /// TODO use an array since it will always be a SHA2 hash (or at least a 32byte hash)
    pub hash: BoundedVec<u8, ConstU32<32>>,
}

impl From<ChainsRaw> for Chains {
    fn from(raw: ChainsRaw) -> Self {
        Self {
            hash: hex_json_value_to_bounded_vec_u8::<32>(&raw.hash),
        }
    }
}

/// TODO update these to support the latest drand API (eg schemeID)
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
            public_key: hex_json_value_to_bounded_vec_u8::<48>(&info.public_key),
            period: info.period,
            genesis_time: info.genesis_time,
            hash: hex_json_value_to_bounded_vec_u8::<32>(&info.hash),
            group_hash: hex_json_value_to_bounded_vec_u8::<32>(&info.group_hash),
        }
    }
}

/// RoundRaw is used for http interactions and JSON parsing with serde.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RoundRaw {
    pub round: u64,
    pub randomness: Value,
    pub previous_signature: Value,
    pub signature: Value,
}

impl Default for RoundRaw {
    fn default() -> Self {
        RoundRaw {
            round: 0,
            randomness: "".into(),
            previous_signature: "".into(),
            signature: "".into(),
        }
    }
}

/// Round will be used in substrate, as it has the Encode/Decode traits derived.
#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, scale_info::TypeInfo)]
pub struct Round {
    pub round: u64,
    // TODO - use array instead of BoundedVec
    pub randomness: BoundedVec<u8, ConstU32<32>>,
    pub previous_signature: BoundedVec<u8, ConstU32<96>>,
    pub signature: BoundedVec<u8, ConstU32<96>>,
}

// TODO Can we do TryFrom instead?
impl From<RoundRaw> for Round {
    fn from(round: RoundRaw) -> Self {
        let RoundRaw {
            round,
            randomness,
            previous_signature,
            signature,
        } = round;

        Round {
            round,
            randomness: hex_json_value_to_bounded_vec_u8::<32>(&randomness),
            previous_signature: hex_json_value_to_bounded_vec_u8::<96>(&previous_signature),
            signature: hex_json_value_to_bounded_vec_u8::<96>(&signature),
        }
    }
}
