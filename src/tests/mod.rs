//! All the tests need to use mocks, since the sp_runtime is only available in the substrate runtime.

use std::{fs::File, io::BufReader};

use sp_io::TestExternalities;
use sp_runtime::offchain::{testing, OffchainWorkerExt};

use drand_verify::derive_randomness;

use crate::{util::hex_to_vec_u8, ChainsRaw, Client, Info, InfoRaw, RoundRaw};

#[test]
fn get_chains() {
    let (offchain, state) = testing::TestOffchainExt::new();
    let mut t = TestExternalities::default();
    t.register_extension(OffchainWorkerExt::new(offchain));

    let client = Client::default();

    let filename = "./src/tests/testdata/chains.json";
    let file = File::open(filename).unwrap();
    let chains: ChainsRaw = serde_json::from_reader(BufReader::new(file)).unwrap();
    let chains_string = serde_json::to_string(&chains).unwrap();
    let expected_response = chains_string.as_bytes();

    t.execute_with(|| {
        state.write().expect_request(testing::PendingRequest {
            method: "GET".into(),
            uri: "http://localhost/chains".into(),
            headers: vec![],
            sent: true,
            response: Some(expected_response.to_vec()),
            ..Default::default()
        });
        let chains = client.chains();
        assert!(chains.is_ok());
    })
}

#[test]
fn get_info() {
    let (offchain, state) = testing::TestOffchainExt::new();
    let mut t = TestExternalities::default();
    t.register_extension(OffchainWorkerExt::new(offchain));

    let client = Client::default();

    let filename = "./src/tests/testdata/chain_info.json";
    let file = File::open(filename).unwrap();
    let info: InfoRaw = serde_json::from_reader(BufReader::new(file)).unwrap();
    let info_string = serde_json::to_string(&info).unwrap();
    let expected_response = info_string.as_bytes();

    t.execute_with(|| {
        state.write().expect_request(testing::PendingRequest {
            method: "GET".into(),
            uri: "http://localhost/info".into(),
            headers: vec![],
            sent: true,
            response: Some(expected_response.to_vec()),
            ..Default::default()
        });
        let info = client.info();
        assert!(info.is_ok());
    })
}

#[test]
fn get_round() {
    let (offchain, state) = testing::TestOffchainExt::new();
    let mut t = TestExternalities::default();
    t.register_extension(OffchainWorkerExt::new(offchain));

    let client = Client::default();

    // the response for "latest" & "round(int)" has the same format, re-use it
    let filename = "./src/tests/testdata/latest.json";
    let file = File::open(filename).unwrap();
    let round: RoundRaw = serde_json::from_reader(BufReader::new(file)).unwrap();
    let round_string = serde_json::to_string(&round).unwrap();
    let expected_response = round_string.as_bytes();

    t.execute_with(|| {
        state.write().expect_request(testing::PendingRequest {
            method: "GET".into(),
            uri: "http://localhost/public/2458190".into(),
            headers: vec![],
            sent: true,
            response: Some(expected_response.to_vec()),
            ..Default::default()
        });
        let round = client.round(2458190);
        assert!(round.is_ok());
        assert_eq!(round.unwrap().round, 2458190);
    })
}

#[test]
fn get_latest() {
    let (offchain, state) = testing::TestOffchainExt::new();
    let mut t = TestExternalities::default();
    t.register_extension(OffchainWorkerExt::new(offchain));

    let client = Client::default();

    let filename = "./src/tests/testdata/latest.json";
    let file = File::open(filename).unwrap();
    let round: RoundRaw = serde_json::from_reader(BufReader::new(file)).unwrap();
    let round_string = serde_json::to_string(&round).unwrap();
    let expected_response = round_string.as_bytes();

    t.execute_with(|| {
        state.write().expect_request(testing::PendingRequest {
            method: "GET".into(),
            uri: "http://localhost/public/latest".into(),
            headers: vec![],
            sent: true,
            response: Some(expected_response.to_vec()),
            ..Default::default()
        });
        let round = client.latest();
        assert!(round.is_ok());
    })
}

#[test]
pub fn verify_randomness() {
    let (offchain, state) = testing::TestOffchainExt::new();
    let mut t = TestExternalities::default();
    t.register_extension(OffchainWorkerExt::new(offchain));

    let client = Client::default();

    // set chain_info for randomness verification
    let chain_info_path = "./src/tests/testdata/chain_info.json";
    let chain_info_file = File::open(chain_info_path).unwrap();
    let chain_info_raw: InfoRaw = serde_json::from_reader(BufReader::new(chain_info_file)).unwrap();
    let chain_info = Info::from(chain_info_raw);

    // get latest round from file, serialize to mock json body
    let latest_round_path = "./src/tests/testdata/latest.json";
    let latest_round_file = File::open(latest_round_path).unwrap();
    let latest_round_raw: RoundRaw =
        serde_json::from_reader(BufReader::new(latest_round_file)).unwrap();
    let latest_round_serialized = serde_json::to_string(&latest_round_raw).unwrap();
    let expected_response = latest_round_serialized.as_bytes();

    t.execute_with(|| {
        state.write().expect_request(testing::PendingRequest {
            method: "GET".into(),
            uri: "http://localhost/public/latest".into(),
            headers: vec![],
            sent: true,
            response: Some(expected_response.to_vec()),
            ..Default::default()
        });
        let round = client.latest().unwrap();
        let randomness = client.verify_randomness(&round, &chain_info.public_key);
        assert!(randomness.is_ok());
    })
}

#[test]
pub fn test_drand_verify_derive_randomness() {
    let signature_vec =
        hex_to_vec_u8("b77042e3ccfeea287f77c956c4321afc0ec1ac4d7c820c1f2106c260587ca67e690700576fc0f465c833dedd503c57470f0b8c5c41dee9ba81dc3e8a16cb75565169676ce9ac4f55bc034a408301e534da0e2d1749add989d19c2bf6bdbff1c4").unwrap();
    let expected_randomness =
        hex_to_vec_u8("ccbdad137f3bc5e01ebd8c7529abc31813a0566b84e6fd765a661398e9bcbc2f").unwrap();

    let derived_randomness = derive_randomness(&signature_vec.as_slice());

    assert_eq!(
        derived_randomness.as_slice(),
        expected_randomness.as_slice()
    );
}
