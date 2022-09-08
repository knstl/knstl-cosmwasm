
wasmd query wasm contract-state smart $DELEGATOR "{\"staked\": {\"address\": \"$(wasmd keys show -a park)\"}}"
wasmd query wasm contract-state smart $DELEGATOR "{\"staked\": {\"address\": \"$(wasmd keys show -a user1)\"}}"
wasmd query wasm contract-state smart $DELEGATOR "{\"staked\": {\"address\": \"$(wasmd keys show -a user2)\"}}"
