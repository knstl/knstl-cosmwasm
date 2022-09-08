print "park"
knstld query bank balances $(knstld keys show -a park)
print "delegator"
knstld query bank balances $DELEGATOR
print "validator1"
knstld query bank balances $(knstld keys show -a validator1)
print "validator2"
knstld query bank balances $(knstld keys show -a validator2)
