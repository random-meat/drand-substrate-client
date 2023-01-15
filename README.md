# drand-substrate-client

This library is a Substrate-specific client for interacting with the drand randomness beacon.
It is meant to be used **only** in a Substrate runtime.

It was loosely inspired by [drand-rs](https://github.com/iprs-dev/drand-rs), which is unmaintained.

Rather than refreshing the aforementioned library, we decided to create a new one that is Substrate-specific. There are a few reasons for this:

- [drand-rs](https://github.com/iprs-dev/drand-rs) uses `std` lib, which is not compatible with `no_std` environment of a substrate runtime.
- Making offchain calls in substrate requires the usage of `sp_runtime::offchain` module.
- We aim to provide minimal functionality to start with, namely interacting with the drand APIs, rather than supporting caching, failover, etc. These may be added in the future if required by the pallet developers.

## Usage

This library can be used in Substrate pallets to interact with any drand network via OFW calls.

Use the `Client` struct to create and configure a drand client. It also has the proper derives to store it directly into Substrate storage, so all nodes have the same configuration of a drand client (network, verification keys, etc).

Docs are inline and can be generated with `cargo doc`, and some examples on how to use it exist in `src/tests`.

## Progress

- [x] Support all [APIs](https://drand.love/developer/http-api/#public-endpoints)
  - [x] `/chains`
  - [x] `/{chain-hash}/info`
  - [x] `/{chain-hash}/public/latest`
  - [x] `/{chain-hash}/public/{round}`
- [x] Integration testing with Substrate offchain worker mock
- [x] Verification of randomness
