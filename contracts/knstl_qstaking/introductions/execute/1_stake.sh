VAL_1=darcvaloper1eyxux4gsvu9t88ey92y6zhcf7p34pyr37350fd
VAL_2=darcvaloper1cm5qg9aweayqx4yqk2ahzpeyajhqfgalx6pknu
VAL_3=darcvaloper1fn9w0ka8x49jrrfpwknzchg6shmltheu2j97gl
STAKE_MSG1="{\"stake\": {\"validator\": \"$VAL_1\"}}"
STAKE_MSG2="{\"stake\": {\"validator\": \"$VAL_2\"}}"
STAKE_MSG3="{\"stake\": {\"validator\": \"$VAL_3\"}}"
wasmd tx wasm execute $DELEGATOR $STAKE_MSG1 --amount 50000udarc --from park  -y --gas 10000000 --fees 60udarc
wasmd tx wasm execute $DELEGATOR $STAKE_MSG2 --amount 22222udarc --from user1 -y --gas 10000000 --fees 60udarc
wasmd tx wasm execute $DELEGATOR $STAKE_MSG3 --amount 33333udarc --from user2 -y --gas 10000000 --fees 60udarc
