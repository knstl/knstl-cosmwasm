# Introduction

## Getting Started

First, you need a daemon executable to interact with chain.
For download, see __[here.](https://github.com/psangwoo/knstld.git)__ 


## Connect with Konstellation Testnet

Once completed `make build`, edit file in $HOME/.knstld/config/client.toml as : 
```
# This is a TOML config file.
# For more information, see https://github.com/toml-lang/toml

###############################################################################
###                           Client Configuration                            ###
###############################################################################

# The network chain ID
chain-id = "parkhub"
# The keyring's backend, where the keys are stored (os|file|kwallet|pass|test|memory)
keyring-backend = "test"
# CLI output format (text|json)
output = "text"
# <host>:<port> to Tendermint RPC interface for this chain
node = "tcp://goz.konstellation.tech:36657"
# Transaction broadcasting mode (sync|async|block)
broadcast-mode = "sync"
```

## Store Cosmwasm Contract to Chain

In this contract, you need 2 additional contract to interact with, `cw20-base` and `qstaking-proxy`.
`cw20-base` can be found [here](https://github.com/CosmWasm/cw-plus/tree/main/contracts/cw20-base), or [here](../../tests) for pre-compiled wasm file.
`qstaking-proxy` can be found [here](../knstl_qstaking_proxy/).

Once you have 3 compiled wasm files, you are ready to store those into chain. Or else, you can use our pre-stored wasm contracts.

Storing wasm file is done via:
```
RES=$(wasmd tx wasm store [wasm_file_name] --from park --gas 10000000 --fees 60udarc -y -b block ) 
&& CODE_ID=$(echo $RES | jq -r '.logs[0].events[-1].attributes[0].value') && echo $CODE_ID
# not working well, will edit properly
```

For our pre-stored wasm contracts, 

`qstaking`: `5`

`qstaking-proxy`: `6`

`cw20-base`: `7`

With these wasm code ids, we can start instantiating ( deploying ).

## Instantiate Contract

First thing to do is instantiate contract, done via:
```
QSTAKING_ID=5
QSTAKING_PROXY_ID=6
CW20_ID=7
INIT_MSG="{\"denom\" : \"udarc\", \"cw20_id\" : \"$CW20_ID\", \"cw20_label\": \"crates.io:cw20-base" \"token_name\": \"qdarc\", \"token_symbol\": \"qdarc\", \"proxy_id\": $QSTAKING_PROXY_ID, \"stake_label\": \"knstl_qstaking_proxy\"}"

knstld tx wasm instantiate $QSTAKING_ID $INIT_MSG --from [user_key_name] --label "knstl_qstaking" -y --fees 40udarc --gas 10000000 -b block --no-admin

DELEGATOR=$(knstld query wasm list-contract-by-code $CONTRACT_NUM --output json | jq -r '.contracts[-1]')

```

This will have variable DELEGATOR set to contract address.

## Execute Contract

Before making any staking interaction between this contract, user has to register to contract's system, which is instantiating [this contract](../knstld_qstaking_proxy/).

Register is done via:

```
REGISTER_MSG="{\"register\": {}}"
knstld tx wasm execute $DELEGATOR $REGISTER_MSG --from [user_name]  --fees 10udarc --gas 10000000 -y
```

After registration, user can execute either 
- Stake
- Unstake ( if staked any )
- Restake
- Claim ( only reward request )
- GetReward
  
Contract execution is done via:
```
STAKE_MSG="{\"stake\": {\"validator\": \"$VAL_1\"}}"
knstld tx wasm execute $DELEGATOR $STAKE_MSG --amount 50000udarc --from [user_name] -y --gas 10000000 --fees 10udarc
REDELEGATE_MSG="{\"change_validator\": {\"from\": \"$VAL_1\", to: \"VAL_2\", amount: \"50000\"},
knstld tx wasm execute $DELEGATOR $REDELEGATE_MSG --amount 50000udarc --from [user_name] -y --gas 10000000 --fees 10udarc
CLAIM_MSG="{\"claim\":{}}"
knstld tx wasm execute $DELEGATOR $CLAIM_MSG --amount 50000udarc --from [user_name] -y --gas 10000000 --fees 10udarc
UNSTAKE_MSG="{\"unstake\": {\"validator\": \"$VAL_2\", \"amount\": \"50000\"}}"
knstld tx wasm execute $DELEGATOR $UNSTAKE_MSG --amount 50000udarc --from [user_name] -y --gas 10000000 --fees 10udarc
GETREWARD_MSG="{\"get_reward\":{}}"
knstld tx wasm execute $DELEGATOR $GETREWARD_MSG --amount 50000udarc --from [user_name] -y --gas 10000000 --fees 10udarc
```

For more details, See [here.](./introductions/)
