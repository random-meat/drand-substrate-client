use std::{fs::File, io::BufReader};

use sp_io::TestExternalities;
use sp_runtime::offchain::{testing, OffchainWorkerExt};

use crate::{ChainsRaw, Client, RoundRaw};

use crate::data_structures::InfoRaw;

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
            uri: "http://localhost/public/2268958".into(),
            headers: vec![],
            sent: true,
            response: Some(expected_response.to_vec()),
            ..Default::default()
        });
        let round = client.round(2268958);
        assert!(round.is_ok());
        assert_eq!(round.unwrap().round, 2268958);
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
