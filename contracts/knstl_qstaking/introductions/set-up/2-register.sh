REGISTER_MSG="{\"register\": {}}"

knstld tx wasm execute $DELEGATOR $REGISTER_MSG --from bridge1  --fees 6udarc --gas 1000000 -y -b block
