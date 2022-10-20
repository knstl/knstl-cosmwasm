CONFIGINFO_QUERY_MSG="{\"config_info\": {}}"
ACCOUNTINFO_QUERY_MSG="{\"account_info\": {\"address\": \"$(knstld keys show -a park)\"}}"
STAKED_QUERY_INFO="{\"staked\": {\"address\": \"$(knstld keys show -a park)\"}}"
TOKENINFO_QUERY_MSG="{\"token_info\": {\"address\": \"$(knstld keys show -a park)\"}}"

knstld query wasm contract-state smart $DELEGATOR $CONFIGINFO_QUERY_MSG
knstld query wasm contract-state smart $DELEGATOR $ACCOUNTINFO_QUERY_MSG
knstld query wasm contract-state smart $DELEGATOR $STAKED_QUERY_INFO
knstld query wasm contract-state smart $DELEGATOR $TOKENINFO_QUERY_MSG
