ALLOWANCE_MSG="{\"increase_allowance\": {\"spender\": \"$DELEGATOR\", \"amount\": \"999999\"}}"
UNSTAKE1_MSG="{\"unstake\": {\"validator\": \"$VAL_2\", \"amount\": \"122222\"}}"
UNSTAKE2_MSG="{\"unstake\": {\"validator\": \"$VAL_3\", \"amount\": \"222222\"}}"
UNSTAKE3_MSG="{\"unstake\": {\"validator\": \"$VAL_1\", \"amount\": \"333333\"}}"
#QDARC=darc1z7asfxkwv0t863rllul570eh5pf2zk07k3d86ag4vtghaue37l5syzfczh
#knstld tx wasm execute $QDARC $ALLOWANCE_MSG --from user  -y --gas 10000000 --fees 60udarc
#knstld tx wasm execute $QDARC $ALLOWANCE_MSG --from user1 -y --gas 10000000 --fees 60udarc
#knstld tx wasm execute $QDARC $ALLOWANCE_MSG --from user2 -y --gas 10000000 --fees 60udarc
sleep 6
knstld tx wasm execute $DELEGATOR $UNSTAKE1_MSG --from park -y  --gas 10000000 --fees 60udarc 
#knstld tx wasm execute $DELEGATOR $UNSTAKE2_MSG --from user1 -y --gas 10000000 --fees 60udarc 
#knstld tx wasm execute $DELEGATOR $UNSTAKE3_MSG --from user2 -y --gas 10000000 --fees 60udarc 
