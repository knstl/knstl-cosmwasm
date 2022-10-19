QSTAKING_ID=1
QSTAKING_PROXY_ID=2
CW20_ID=3
INIT_MSG="{\"denom\" : \"udarc\", \"cw20_id\" : $CW20_ID, \"cw20_label\": \"crates.io:cw20-base\", \"token_name\": \"qdarc\", \"token_symbol\": \"qdarc\", \"proxy_id\": $QSTAKING_PROXY_ID, \"proxy_label\": \"knstl_qstaking_proxy\", \"commission_rate\": \"0.15\", \"community_pool\": \"$COMMUNITY_POOL\", \"unbond_period\": 120}"

knstld tx wasm instantiate $QSTAKING_ID $INIT_MSG --from park --label "knstl_qstaking" -y --fees 6udarc --gas 1000000 -b block --no-admin

DELEGATOR=$(knstld query wasm list-contract-by-code $QSTAKING_ID --output json | jq -r '.contracts[-1]')
