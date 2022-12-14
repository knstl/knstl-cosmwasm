# Q-Staking Contract

This smart contract provides a feature of staking native token and get immediate rewards.

## Running this contract

You will need Rust 1.44.1+ with `wasm32-unknown-unknown` target installed.

You can run unit tests on this via: 

`cargo test`

Or you can checkout schemas via:

`cargo schema`

Once you are happy with the content, you can compile it to wasm via:

```
RUSTFLAGS='-C link-arg=-s' cargo wasm
cp ../../target/wasm32-unknown-unknown/release/knstl_qstaking.wasm .
ls -l qstaking.wasm
sha256sum qstaking.wasm
```

Or for a production-ready (compressed) build, run the following from the __repository root__:

```
docker run --rm -v "$(pwd)":/code \
  --mount type=volume,source="$(basename "$(pwd)")_knstl_qstaking_cache",target=/code/contracts/knstl_qstaking/target \
  --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
  cosmwasm/rust-optimizer:0.12.7 ./contracts/knstl_qstaking
  ```

## About This Contract

This smart contract lets user stake to validator that user chose, with additional CW20 token subsidized right away.

With this contract, user can stake their tokens, without any reward loss, and still token being liquid.

## How-to Use

For basic knowledges, see __[here.](https://docs.cosmwasm.com/docs/1.0/getting-started/interact-with-contract)__

For specifications, see __[here.](./SPEC.md)__
