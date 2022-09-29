WITHDRAW_MSG1="{\"withdraw\": {\"validator\": \"$VAL_1\"}}"
WITHDRAW_MSG2="{\"withdraw\": {\"validator\": \"$VAL_2\"}}"
WITHDRAW_MSG3="{\"withdraw\": {\"validator\": \"$VAL_2\"}}"
WITHDRAWALL_MSG="{\"withdraw_all\": {}}"


knstld tx wasm execute $DELEGATOR $WITHDRAW_MSG1 --from user -y --gas 1000000 --fees 6udarc
knstld tx wasm execute $DELEGATOR $WITHDRAW_MSG2 --from user -y --gas 1000000 --fees 6udarc
knstld tx wasm execute $DELEGATOR $WITHDRAW_MSG3 --from user -y --gas 1000000 --fees 6udarc
knstld tx wasm execute $DELEGATOR $WITHDRAWALL_MSG --from user -y --gas 1000000 --fees 6udarc