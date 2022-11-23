#![cfg_attr(not(feature = "std"), no_std)]

use sp_runtime::offchain::{
    http::{Error, Request},
    Duration,
};

use drand_verify::g1_from_variable;

use scale_info::prelude::format;
use sp_runtime::{traits::ConstU32, BoundedVec, RuntimeDebug};
use sp_std::str;
use sp_std::vec::Vec;

pub use crate::data_structures::*;

/// drand client errors
#[derive(RuntimeDebug)]
pub enum ClientError {
    /// TODO
    Http,
    /// TODO
    Json,
    Unknown,
    /// The round was was unable to be verified
    RandomnessVerificationError,
    /// `chain_info` is not configured. Set it with `set_chain_info(Info)`
    ChainNotConfigured,
    /// TODO catch all error
    Misc,
    /// Signature verification failed
    InvalidSignature,
}

/// drand client configuration
pub struct Config {}

/// Client is a wrapper around the offchain http client.
/// TODO This should include the chain's `Info` struct as a field, instead of just parts of it.
pub struct Client {
    /// depreciate chain_hash, use chain_info.hash instead
    chain_hash: Option<Vec<u8>>,
    config: Config,
    endpoint: Vec<u8>,
    chain_info: Option<Info>,
    /// Store latest round to prevent old randomness from being used.
    // TODO should we calculate what this should be based on genesis_time and current time?
    latest_round: u64,
}

// {"public_key":"868f005eb8e6e4ca0a47c8a77ceaa5309a47978a7c71bc5cce96366b5d7a569937c529eeda66c7293784a9402801af31",
// "period":30,
// "genesis_time":1595431050,
// "hash":"8990e7a9aaed2ffed73dbd7092123d6f289930540d7651336225dc172e51b2ce",
// "groupHash":"176f93498eac9ca337150b46d21dd58673ea4e3581185f869672e59fa4cb390a",
// "schemeID":"pedersen-bls-chained",
// "metadata":{"beaconID":"default"}}

impl Default for Client {
    #[cfg(not(test))]
    fn default() -> Self {
        use util::hex_to_vec_u8;

        let chain_hash =
            hex_to_vec_u8("8990e7a9aaed2ffed73dbd7092123d6f289930540d7651336225dc172e51b2ce")
                .unwrap();

        Client {
            config: Config {},
            endpoint: "https://drand.cloudflare.com".as_bytes().to_vec(),
            chain_hash: Some(chain_hash.clone()),
            chain_info: Some(
                Info {
                    public_key: hex_to_vec_u8("868f005eb8e6e4ca0a47c8a77ceaa5309a47978a7c71bc5cce96366b5d7a569937c529eeda66c7293784a9402801af31").unwrap().try_into().unwrap(),
                    period: 30,
                    genesis_time: 1595431050,
                    hash: chain_hash.try_into().unwrap(),
                    group_hash: hex_to_vec_u8("176f93498eac9ca337150b46d21dd58673ea4e3581185f869672e59fa4cb390a").unwrap().try_into().unwrap(),
                }
            ),
            latest_round: 2457230, // as of 2022-11-22
        }
    }

    #[cfg(test)]
    fn default() -> Self {
        Client {
            config: Config {},
            endpoint: "http://localhost".as_bytes().to_vec(),
            chain_hash: None,
            chain_info: None,
            latest_round: 0,
        }
    }
}

impl Client {
    pub fn chains(&self) -> Result<ChainsRaw, Error> {
        let mut url_str = self.endpoint.clone();
        url_str.extend("/chains".as_bytes().to_vec());
        let body = self.make_request(url_str).unwrap();

        // Create a str slice from the body.
        let body_str = sp_std::str::from_utf8(&body).map_err(|_| {
            log::warn!("No UTF8 body");
            Error::Unknown
        })?;

        log::info!("Response: {}", body_str);

        let chains: ChainsRaw = serde_json::from_str(body_str).map_err(|_| {
            log::warn!("Failed to deserialize");
            Error::Unknown
        })?;

        Ok(chains)
    }

    pub fn info(&self) -> Result<InfoRaw, Error> {
        let mut url_str = self.endpoint.clone();
        url_str.extend("/info".as_bytes().to_vec());
        let body = self.make_request(url_str).unwrap();

        // Create a str slice from the body.
        let body_str = sp_std::str::from_utf8(&body).map_err(|_| {
            log::warn!("No UTF8 body");
            Error::Unknown
        })?;

        log::info!("Response: {}", body_str);

        let info: InfoRaw = serde_json::from_str(body_str).map_err(|_| {
            log::warn!("Failed to deserialize");
            Error::Unknown
        })?;

        Ok(info)
    }

    /// Associates the client to a specific chain. Required to verify randomness.
    pub fn set_chain_info(&mut self, chain_info: Info) -> Result<(), ClientError> {
        // TODO finish parsing other fields
        // Make sure public key is a valid key before we store so we can use unchecked
        let _public_key =
            g1_from_variable(chain_info.public_key.as_slice()).map_err(|_| ClientError::Misc)?;

        self.chain_info = Some(chain_info);
        Ok(())
    }

    pub fn chain_info(&self) -> Result<Info, ClientError> {
        match &self.chain_info {
            Some(info) => Ok(info.clone()),
            None => Err(ClientError::ChainNotConfigured),
        }
    }

    // TODO Change all to return parsed, not Raw
    pub fn round(&self, round: u64) -> Result<RoundRaw, Error> {
        let mut url_str = self.endpoint.clone();
        url_str.extend(format!("/public/{}", round).as_bytes().to_vec());
        let body = self.make_request(url_str).unwrap();

        // Create a str slice from the body.
        let body_str = sp_std::str::from_utf8(&body).map_err(|_| {
            log::warn!("No UTF8 body");
            Error::Unknown
        })?;

        log::info!("Response: {}", body_str);

        let round: RoundRaw = serde_json::from_str(body_str).map_err(|_| {
            log::warn!("Failed to deserialize");
            Error::Unknown
        })?;

        Ok(round)
    }

    pub fn latest(&self) -> Result<Round, Error> {
        let mut url_str = self.endpoint.clone();
        url_str.extend("/public/latest".as_bytes().to_vec());
        let body = self.make_request(url_str).unwrap();

        // Create a str slice from the body.
        let body_str = sp_std::str::from_utf8(&body).map_err(|_| {
            log::warn!("No UTF8 body");
            Error::Unknown
        })?;

        log::info!("Response: {}", body_str);

        let round_raw: RoundRaw = serde_json::from_str(body_str).map_err(|_| {
            log::warn!("Failed to deserialize");
            Error::Unknown
        })?;

        Ok(Round::from(round_raw))
    }

    pub fn make_request(&self, url: Vec<u8>) -> Result<Vec<u8>, Error> {
        // We want to keep the offchain worker execution time reasonable, so we set a hard-coded
        // deadline to 2s to complete the external call.
        // You can also wait idefinitely for the response, however you may still get a timeout
        // coming from the host machine.
        let send_deadline = sp_io::offchain::timestamp().add(Duration::from_millis(2_000));
        // Initiate an external HTTP GET request.
        // This is using high-level wrappers from `sp_runtime`, for the low-level calls that
        // you can find in `sp_io`. The API is trying to be similar to `reqwest`, but
        // since we are running in a custom WASM execution environment we can't simply
        // import the library here.
        let url_str = unsafe { str::from_utf8_unchecked(&url) };
        let request = Request::get(url_str);
        // We set the deadline for sending of the request, note that awaiting response can
        // have a separate deadline. Next we send the request, before that it's also possible
        // to alter request headers or stream body content in case of non-GET requests.
        let pending = request
            .deadline(send_deadline)
            .send()
            .map_err(|_| Error::IoError)?;

        // The request is already being processed by the host, we are free to do anything
        // else in the worker (we can send multiple concurrent requests too).
        // At some point however we probably want to check the response though,
        // so we can block current thread and wait for it to finish.
        // Note that since the request is being driven by the host, we don't have to wait
        // for the request to have it complete, we will just not read the response.

        // TODO: Right now the response deadline is same as send deadline, but maybe let's make them differnet later.
        let response_deadline = sp_io::offchain::timestamp().add(Duration::from_millis(2_000));
        let response = pending
            .try_wait(response_deadline)
            .map_err(|_| Error::DeadlineReached)??;
        // Let's check the status code before we proceed to reading the response.
        if response.code != 200 {
            log::warn!("Unexpected status code: {}", response.code);
            return Err(Error::Unknown);
        }

        // Next we want to fully read the response body and collect it to a vector of bytes.
        // Note that the return object allows you to read the body in chunks as well
        // with a way to control the deadline.
        let body = response.body().collect::<Vec<u8>>();
        Ok(body)
    }

    pub fn verify_randomness(
        &self,
        round: &Round,
        pub_key_vec: &BoundedVec<u8, ConstU32<48>>,
    ) -> Result<BoundedVec<u8, ConstU32<32>>, ClientError> {
        // verify signature
        let Round {
            round,
            randomness,
            previous_signature,
            signature,
        } = round;

        let pk_point = g1_from_variable(pub_key_vec.as_slice()).map_err(|_| ClientError::Misc)?;
        match drand_verify::verify(&pk_point, round.clone(), previous_signature, signature) {
            Ok(b) => {
                if !b {
                    return Err(ClientError::InvalidSignature);
                } else {
                    Ok(randomness.clone())
                }
            }
            Err(_) => Err(ClientError::RandomnessVerificationError),
        }
    }
}

mod data_structures;

#[cfg(all(test, feature = "std"))]
mod tests;

pub mod util;
