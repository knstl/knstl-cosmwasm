# Specification

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

`qstaking`: `34`

`qstaking-proxy`: `33`

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
CosmWasm Smart Contract Instantiation creates a contract address, which can be queried with : 
```
knstld query wasm list-contract-by-code [contract-num]
```

Now with this contract address, we can interact with the contract.

CosmWasm Smart Contract execution is done via:
```
knstld tx wasm execute [contract-address] [json-msg] --from [user-name] --gas [gas-amount] --fees [gas-fee-amount] --amount [amount-to-send-to-contract]
```
Followings are executable commands : 
- Stake { validator: String }
  - Stake to `validator`. Amount to stake is set by --amount flag on execution.
- Unstake { validator: String, amount: Uint128 }
  - Unstake `amount` from `validator`.
- Restake { from: String, to: String, amount: Uint128 }
  - Change delegation amount of `amount` on `from` validator to `to` validator.
- Claim { }
  - Claim rewards. Errors out when no unbonded tokens.
- Withdraw { validator: String }
  - Withdraw rewards from `validator` to `proxy-contract`.
- WithdrawAll { }
  - Withdraw rewards from all validators user staked to `proxy-contract`.
- Compound { validator: String, amount: Uint128 }
  - Stake `amount` to `validator`, using `proxy-contract`'s balance.
  

Before making any staking interaction to this contract, user has to register to contract's system, which is instantiating [this contract](../knstld_qstaking_proxy/).

Register is done via:

```
REGISTER_MSG="{\"register\": {}}"
knstld tx wasm execute $DELEGATOR $REGISTER_MSG --from [user_name] --fees 10udarc --gas 10000000 -y
```
After registration, now user can interact with this contract.
JSON execution messages for this contract :
```
REGISTER_MSG="{\"register\": {}}"
STAKE_MSG="{\"stake\": {\"validator\": \"[validator-address]]\"}}" # for staking amount, use --amount flag
UNSTAKE_MSG="{\"unstake\": {\"validator\": \"[validator-address]\", \"amount\": \"[amount-to-unstake]\"}}"
RESTAKE_MSG="{\"restake\": {\"validator\": \"[validator-address]\"}}"
CLAIM_MSG="{\"claim\": {}}"
WITHDRAW_MSG="{\"withdraw\": {\"validator\": \"[validator-address]\"}}"
WITHDRAWALL_MSG="{\"withdraw_all\": {}}"
COMPOUND_MSG="{\"compound\": {\"validator\": \"[validator-address]\", \"amount\": \"[amount-to-unstake]\"}}"
```

For shell scripts of usage scenarios, See [here.](./introductions/)

## Query Contract
Users can query to CosmWasm Smart Contract for states saved in it.

CosmWasm Smart Contract query is done via:
```
knstld query wasm contract-state smart [contract-address] [json-msg]
```
Followings are queriable commands : 
- ConfigInfo { }
  - Return contract's config information.
- AccountInfo { address : Addr }
  - Return whether `address` is registered or not.
- Staked { address : Addr }
  - Return `address`'s staking status.
- TokenInfo { address : Addr }
  - Return `address`'s cw20 token amount - which is qDARC..

`Addr` is same type with `String`.

JSON query messages for this contract :
```
CONFIGINFO_QUERY_MSG="{\"config_info\": {}}"
ACCOUNTINFO_QUERY_MSG="{\"account_info\": {\"address\": \"[user-address]\"}}"
STAKED_QUERY_INFO="{\"staked\": {\"address\": \"[user-address]\"}}"
TOKENINFO_QUERY_MSG="{\"token_info\": {\"address\": \"[user-address]\"}}"
```

For shell scripts of usage scenarios, See [here.](./introductions/)