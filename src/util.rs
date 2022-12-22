extern crate alloc;

use alloc::vec::Vec;
use sp_runtime::{traits::ConstU32, BoundedVec};

// return Option since Error is part of std
pub fn hex_to_vec_u8(s: &str) -> Option<Vec<u8>> {
    if s.len() % 2 == 0 {
        (0..s.len())
            .step_by(2)
            .map(|i| {
                s.get(i..i + 2)
                    .and_then(|sub| u8::from_str_radix(sub, 16).ok())
            })
            .collect()
    } else {
        None
    }
}

// TODO `impl TryFrom<Value> for BoundedVec...` instead
pub fn hex_json_value_to_bounded_vec_u8<const S: u32>(
    val: &serde_json::value::Value,
) -> BoundedVec<u8, ConstU32<S>> {
    // TODO remove unwraps (runtime should not panic)
    let bytes = hex_to_vec_u8(val.as_str().unwrap()).unwrap();

    bytes.try_into().unwrap()
}

#[test]
fn test_hex_to_vec_u8_uneven() {
    let hex = "ccbdad137f3bc5e01ebd8c7529abc31813a0566b84e6fd765a661398e9bcbc2";
    let bytes = hex_to_vec_u8(hex);
    assert!(bytes.is_none());
}

#[test]
fn test_hex_to_vec_u8() {
    let hex = "ccbdad137f3bc5e01ebd8c7529abc31813a0566b84e6fd765a661398e9bcbc2f";
    let bytes = hex_to_vec_u8(hex).unwrap();
    assert_eq!(bytes.len(), 32);
}
