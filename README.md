# drand-substrate

## testing no_std builds on M1 Mac

https://github.com/rust-bitcoin/rust-secp256k1/issues/283#issuecomment-1200858455

```
AR=/usr/local/opt/llvm/bin/llvm-ar CC=/usr/local/opt/llvm/bin/clang cargo build --release --target wasm32-unknown-unknown
```