#![cfg_attr(not(feature = "std"), no_std)]

use sp_runtime::offchain::{
    http::{Error, Request},
    Duration,
};

use sp_std::str;
use sp_std::vec::Vec;

use crate::data_structures::Info;

pub struct Config {}

pub struct Client {
    chainHash: Option<Vec<u8>>,
    config: Config,
    endpoint: Vec<u8>,
}

impl Default for Client {
    #[cfg(not(test))]
    fn default() -> Self {
        Client {
            config: Config {},
            endpoint: "https://api.drand.sh".as_bytes().to_vec(),
            chainHash: Some(
                "8990e7a9aaed2ffed73dbd7092123d6f289930540d7651336225dc172e51b2ce".as_bytes().to_vec(),
            ),
        }
    }

    #[cfg(test)]
    fn default() -> Self {
        Client {
            config: Config {},
            endpoint: "http://localhost".as_bytes().to_vec(),
            chainHash: None,
        }
    }
}

impl Client {
    // TODO should return an instance of the Info struct
    pub fn info(&self) -> Result<(), Error> {
        let mut url_str = self.endpoint.clone();
        url_str.extend("/info".as_bytes().to_vec());
        let body = self.make_request(url_str).unwrap();

        // Create a str slice from the body.
        let body_str = sp_std::str::from_utf8(&body).map_err(|_| {
            log::warn!("No UTF8 body");
            Error::Unknown
        })?;

        log::info!("Response: {}", body_str);

        let val: Info = serde_json::from_str(body_str).map_err(|_| {
            log::warn!("Failed to deserialize");
            Error::Unknown
        })?;
        // let val = lite_json::parse_json(body_str);
        // assert!(val.is_ok(), "Invalid JSON");

        // some example of using the lite_json library to parse the JSON. should adapt to Info struct
        // let price = match val.ok()? {
        //     JsonValue::Object(obj) => {
        //         let (_, v) = obj
        //             .into_iter()
        //             .find(|(k, _)| k.iter().copied().eq("USD".chars()))?;
        //         match v {
        //             JsonValue::Number(number) => number,
        //             _ => return None,
        //         }
        //     }
        //     _ => return None,
        // };

        Ok(())
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
}

mod data_structures;

#[cfg(test)]
mod tests;
