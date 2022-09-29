RESTAKE_MSG="{\"restake\": {\"from\": \"$VAL_1\", \"to\": \"$VAL_2\", \"amount\": \"50000\"}}"
knstld tx wasm execute $DELEGATOR $RESTAKE_MSG --from user -y --gas 1000000 --fees 6udarc