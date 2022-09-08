
INIT_MSG="{\"tokencontract_id\" : 2, \"tokencontract_label\" : \"crates.io:cw20-base\", \"token_name\": \"qdarc\", \"token_symbol\": \"qdarc\", \"denom\": \"udarc\", \"reward_denom\": \"\", \"reward_contract\": \"\", \"stake_id\": 3, \"stake_label\": \"knstl_staker\"}"
CONTRACT_NUM=1
#knstld tx wasm instantiate 2 "{\"name\": \"qdarc\", \"symbol\": \"qdarc\", \"decimals\": 6, \"initial_balances\": [], \"mint\": {\"minter\": \"$(knstld keys show -a park)\"}}" --from park --label "crates.io:cw20-base" -y --fees 2udarc --gas auto --admin $(knstld keys show -a park)
wasmd tx wasm instantiate $CONTRACT_NUM $INIT_MSG --from park --label "knstl_delegator" -y --fees 34udarc --gas 5555555 -b block --no-admin

DELEGATOR=$(wasmd query wasm list-contract-by-code $CONTRACT_NUM --output json | jq -r '.contracts[-1]')
