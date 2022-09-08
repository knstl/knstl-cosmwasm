# Q-Staking-Proxy Contract

This smart contract provides a feature of staking native token and get immediate rewards.

## Running this contract

You will need Rust 1.44.1+ with `wasm32-unknown-unknown` target installed.

You can run unit tests on this via: 

`cargo test`

Once you are happy with the content, you can compile it to wasm via:

```
RUSTFLAGS='-C link-arg=-s' cargo wasm
cp ../../target/wasm32-unknown-unknown/release/knstl_qstaking_proxy.wasm .
ls -l qstaking_proxy.wasm
sha256sum qstaking_proxy.wasm
```

Or for a production-ready (compressed) build, run the following from the __repository root__:

```
docker run --rm -v "$(pwd)":/code \
  --mount type=volume,source="$(basename "$(pwd)")_knstl_qstaking_proxy_cache",target=/code/contracts/knstl_qstaking_proxy/target \
  --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
  cosmwasm/rust-optimizer:0.12.7 ./contracts/knstl_qstaking_proxy
  ```

## About This Contract

This smart contract is a proxy-contract which can only be called by the one who instantiated, who will be knstld_qstaking in design.

Having this contract, it makes qstaking contract able to create an address for q-staking interactions.

For more details, See __[Here](../knstl_qstaking/README.md)__