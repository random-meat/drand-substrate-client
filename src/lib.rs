#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(all(test, feature = "std"))]
mod tests;

mod data_structures;
mod util;

pub use crate::data_structures::*;

use sp_runtime::offchain::{
    http::{Error, Request},
    Duration,
};

use drand_verify::g1_from_variable;

use codec::{Decode, Encode};
use scale_info::prelude::format;
use serde::{Deserialize, Serialize};
use sp_runtime::{traits::ConstU32, BoundedVec, RuntimeDebug};
use sp_std::str;
use sp_std::vec::Vec;

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

/// Client is a wrapper around the offchain http client.
/// Configuration used to specify the root of trust (chain info) and some endpoints that should be running drand nodes.
/// This will be stored on-chain.
#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, scale_info::TypeInfo)]
pub struct Client {
    pub endpoint: BoundedVec<u8, ConstU32<100>>,
    pub chain_info: Option<Info>,
}

impl Default for Client {
    /// League of Entropy base urls and chain info (from https://drand.cloudflare.com/info)
    #[cfg(not(test))]
    fn default() -> Self {
        Client {
            endpoint: "https://drand.cloudflare.com"
                .as_bytes()
                .to_owned()
                .try_into()
                .unwrap(),
            chain_info: Some(Info::default()),
        }
    }

    #[cfg(test)]
    fn default() -> Self {
        Client {
            endpoint: "http://localhost".as_bytes().to_owned().try_into().unwrap(),
            chain_info: Some(Info::default()),
        }
    }
}

impl Client {
    // Creates a new drand client with the provided URL and chain_info.
    pub fn new(url: &str, chain_info: Info) -> Self {
        Client {
            endpoint: url.as_bytes().to_owned().to_vec().try_into().unwrap(),
            chain_info: Some(chain_info),
        }
    }

    /// HTTP GET on the chain's `/chains` endpoint.
    pub fn chains(&self) -> Result<Chains, Error> {
        let mut url_str = self.endpoint.clone().to_vec();
        url_str.extend("/chains".as_bytes().to_vec());
        let body = self.make_request(url_str).unwrap();

        // Create a str slice from the body.
        let body_str = sp_std::str::from_utf8(&body).map_err(|_| {
            log::warn!("No UTF8 body");
            Error::Unknown
        })?;

        log::info!("Response: {}", body_str);

        let chains_raw: ChainsRaw = serde_json::from_str(body_str).map_err(|_| {
            log::warn!("Failed to deserialize");
            Error::Unknown
        })?;

        Ok(Chains::from(chains_raw))
    }

    /// HTTP GET on the chain's `/info` endpoint.
    pub fn info(&self) -> Result<Info, Error> {
        let mut url_str = self.endpoint.clone().to_vec();
        url_str.extend("/info".as_bytes().to_vec());
        let body = self.make_request(url_str).unwrap();

        // Create a str slice from the body.
        let body_str = sp_std::str::from_utf8(&body).map_err(|_| {
            log::warn!("No UTF8 body");
            Error::Unknown
        })?;

        log::info!("Response: {}", body_str);

        let info_raw: InfoRaw = serde_json::from_str(body_str).map_err(|_| {
            log::warn!("Failed to deserialize");
            Error::Unknown
        })?;

        Ok(Info::from(info_raw))
    }

    /// Associates the client to a specific chain. Required to verify randomness.
    pub fn set_chain_info(&mut self, info: Info) -> () {
        self.chain_info = Some(info);
    }

    /// Getter for self.chain_info
    pub fn chain_info(&self) -> Result<Info, ClientError> {
        match &self.chain_info {
            Some(info) => Ok(info.clone()),
            None => Err(ClientError::ChainNotConfigured),
        }
    }

    /// Queries and verifies a round of randomness.
    pub fn round(&self, round: u64) -> Result<Round, Error> {
        let mut url_str = self.endpoint.clone().to_vec();
        url_str.extend(format!("/public/{round}").as_bytes().to_vec());
        let body = self.make_request(url_str).unwrap();

        // Create a str slice from the body.
        let body_str = sp_std::str::from_utf8(&body).map_err(|_| {
            log::warn!("No UTF8 body");
            Error::Unknown
        })?;

        log::info!("Response: {}", body_str);

        let round = Round::from(RoundRaw::from(serde_json::from_str(body_str).map_err(
            |_| {
                log::warn!("Failed to deserialize");
                Error::Unknown
            },
        )?));

        // TODO fix unwrap
        Client::verify_randomness(&round, &self.chain_info.clone().unwrap().public_key)
            .map_err(|_| Error::Unknown)?;

        Ok(round)
    }

    /// This fetches the latest round from the drand server.
    pub fn latest(&self) -> Result<Round, Error> {
        let mut url_str = self.endpoint.clone().to_vec();
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

    /// Static function to verify a round against a public key
    pub fn verify_randomness(
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
        match drand_verify::verify(&pk_point, *round, previous_signature, signature) {
            Ok(b) => {
                if !b {
                    Err(ClientError::InvalidSignature)
                } else {
                    Ok(randomness.clone())
                }
            }
            Err(_) => Err(ClientError::RandomnessVerificationError),
        }
    }
}
