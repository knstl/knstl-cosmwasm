REGISTER_MSG="{\"register\": {}}"

wasmd tx wasm execute $DELEGATOR $REGISTER_MSG --from park  --fees 100udarc --gas 10000000 -y -b block
wasmd tx wasm execute $DELEGATOR $REGISTER_MSG --from user1 --fees 100udarc --gas 10000000 -y -b block
wasmd tx wasm execute $DELEGATOR $REGISTER_MSG --from user2 --fees 100udarc --gas 10000000 -y -b block
	
