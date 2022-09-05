use std::{fs::File, io::BufReader};

use sp_io::TestExternalities;
use sp_runtime::offchain::{testing, OffchainWorkerExt};

use crate::Client;

use crate::data_structures::Info;

#[test]
fn get_info() {
    let (offchain, state) = testing::TestOffchainExt::new();
    let mut t = TestExternalities::default();
    t.register_extension(OffchainWorkerExt::new(offchain));

    let client = Client::default();

    let filename = "./src/tests/testdata/chain_info.json";
    let file = File::open(filename).unwrap();
    let info: Info = serde_json::from_reader(BufReader::new(file)).unwrap();
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
