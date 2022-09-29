REGISTER_MSG="{\"register\": {}}"

knstld tx wasm execute $DELEGATOR $REGISTER_MSG --from user  --fees 6udarc --gas 1000000 -y -b block