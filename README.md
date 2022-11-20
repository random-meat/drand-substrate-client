# drand-substrate-client

This library is a substrate-specific client for interacting with the drand randomness beacon.
It is meant to be used **only** in a substrate runtime.

It was loosely inspired by [drand-rs](https://github.com/iprs-dev/drand-rs), which is unmaintained.

Rather than refreshing the aforementioned library, we decided to create a new one that is substrate-specific. There are a few reasons for this:
- [drand-rs](https://github.com/iprs-dev/drand-rs) uses `std` lib, which is not compatible with `no_std` environment of a substrate runtime.
- Making offchain calls in substrate requires the usage of `sp_runtime::offchain` module.
- We aim to provide minimal functionality to start with, namely interacting with the drand APIs, rather than supporting caching, failover, etc. These may be added in the future if required by the pallet developers.

## Usage

## Progress

- [x] Support all [APIs](https://drand.love/developer/http-api/#public-endpoints)
    - [x] `/chains`
    - [x] `/{chain-hash}/info`
    - [x] `/{chain-hash}/public/latest`
    - [x] `/{chain-hash}/public/{round}`
- [ ] Support multiple endpoints (e.g. https://api3.drand.sh)
- [x] Testing with an offchain worker mock
- [ ] Verification of randomness