extern crate alloc;

use alloc::vec::Vec;
use sp_runtime::{traits::ConstU32, BoundedVec};

pub fn hex_to_bytes(s: &str) -> Option<Vec<u8>> {
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

pub fn str_to_bounded_vec<const S: u32>(
    val: &serde_json::value::Value,
) -> BoundedVec<u8, ConstU32<S>> {
    let bytes = hex_to_bytes(val.as_str().unwrap()).unwrap();
    let bounded_vec = bytes.try_into().unwrap();
    bounded_vec
}

#[test]
fn test_hex_to_bytes_uneven() {
    let hex = "ccbdad137f3bc5e01ebd8c7529abc31813a0566b84e6fd765a661398e9bcbc2";
    let bytes = hex_to_bytes(hex);
    assert!(bytes.is_none());
}

#[test]
fn test_hex_to_bytes() {
    let hex = "ccbdad137f3bc5e01ebd8c7529abc31813a0566b84e6fd765a661398e9bcbc2f";
    let bytes = hex_to_bytes(hex).unwrap();
    assert_eq!(bytes.len(), 32);
}
