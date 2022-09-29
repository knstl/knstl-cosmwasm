# to check validator address:
# knstld query staking validators
VAL_1=darcvaloper1eyxux4gsvu9t88ey92y6zhcf7p34pyr37350fd
VAL_2=darcvaloper1cm5qg9aweayqx4yqk2ahzpeyajhqfgalx6pknu
VAL_3=darcvaloper1fn9w0ka8x49jrrfpwknzchg6shmltheu2j97gl
STAKE_MSG1="{\"stake\": {\"validator\": \"$VAL_1\"}}"
STAKE_MSG2="{\"stake\": {\"validator\": \"$VAL_2\"}}"
STAKE_MSG3="{\"stake\": {\"validator\": \"$VAL_3\"}}"
knstld tx wasm execute $DELEGATOR $STAKE_MSG1 --amount 50000udarc --from user -y --gas 1000000 --fees 6udarc
knstld tx wasm execute $DELEGATOR $STAKE_MSG2 --amount 50000udarc --from user -y --gas 1000000 --fees 6udarc
knstld tx wasm execute $DELEGATOR $STAKE_MSG3 --amount 50000udarc --from user -y --gas 1000000 --fees 6udarc