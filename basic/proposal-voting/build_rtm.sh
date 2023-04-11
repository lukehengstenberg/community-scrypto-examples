# # Getting the current script dir
SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )
echo $SCRIPT_DIR

# Resetting resim
resim reset

# Creating some accounts
OP1=$(resim new-account)
export CHAIRPERSON_PRIV_KEY=$(echo "$OP1" | sed -nr "s/Private key: ([[:alnum:]_]+)/\1/p")
export CHAIRPERSON_PUB_KEY=$(echo "$OP1" | sed -nr "s/Public key: ([[:alnum:]_]+)/\1/p")
export CHAIRPERSON_ADDRESS=$(echo "$OP1" | sed -nr "s/Account component address: ([[:alnum:]_]+)/\1/p")
export CHAIRPERSON_OWNER_BADGE=$(echo "$OP1" | sed -nr "s/Owner badge: ([[:alnum:]_]+)/\1/p")
echo "[•] Created Chairperson Account: address-$CHAIRPERSON_ADDRESS, private_key-$CHAIRPERSON_PRIV_KEY, 
    owner_badge-$CHAIRPERSON_OWNER_BADGE"

OP2=$(resim new-account)
export VOTER1_PRIV_KEY=$(echo "$OP2" | sed -nr "s/Private key: ([[:alnum:]_]+)/\1/p")
export VOTER1_PUB_KEY=$(echo "$OP2" | sed -nr "s/Public key: ([[:alnum:]_]+)/\1/p")
export VOTER1_ADDRESS=$(echo "$OP2" | sed -nr "s/Account component address: ([[:alnum:]_]+)/\1/p")
export VOTER1_OWNER_BADGE=$(echo "$OP1" | sed -nr "s/Owner badge: ([[:alnum:]_]+)/\1/p")
echo "[•] Created Voter1 Account: address-$VOTER1_ADDRESS, private_key-$VOTER1_PRIV_KEY, 
    owner_badge-$VOTER1_OWNER_BADGE"

OP3=$(resim new-account)
export VOTER2_PRIV_KEY=$(echo "$OP3" | sed -nr "s/Private key: ([[:alnum:]_]+)/\1/p")
export VOTER2_PUB_KEY=$(echo "$OP3" | sed -nr "s/Public key: ([[:alnum:]_]+)/\1/p")
export VOTER2_ADDRESS=$(echo "$OP3" | sed -nr "s/Account component address: ([[:alnum:]_]+)/\1/p")
export VOTER2_OWNER_BADGE=$(echo "$OP1" | sed -nr "s/Owner badge: ([[:alnum:]_]+)/\1/p")
echo "[•] Created Voter2 Account: address-$VOTER2_ADDRESS, private_key-$VOTER2_PRIV_KEY, 
    owner_badge-$VOTER2_OWNER_BADGE"

OP4=$(resim new-account)
export VOTER3_PRIV_KEY=$(echo "$OP4" | sed -nr "s/Private key: ([[:alnum:]_]+)/\1/p")
export VOTER3_PUB_KEY=$(echo "$OP4" | sed -nr "s/Public key: ([[:alnum:]_]+)/\1/p")
export VOTER3_ADDRESS=$(echo "$OP4" | sed -nr "s/Account component address: ([[:alnum:]_]+)/\1/p")
export VOTER3_OWNER_BADGE=$(echo "$OP1" | sed -nr "s/Owner badge: ([[:alnum:]_]+)/\1/p")
echo "[•] Created Voter3 Account: address-$VOTER3_ADDRESS, private_key-$VOTER3_PRIV_KEY, 
    owner_badge-$VOTER3_OWNER_BADGE"

# Setting the first account as the default account, FIX THIS NEEDS AN OWNER BADGE
resim set-default-account $CHAIRPERSON_ADDRESS $CHAIRPERSON_PRIV_KEY $CHAIRPERSON_OWNER_BADGE

# Publishing the package
PK_OP=$(resim publish ".")
export PACKAGE=$(echo "$PK_OP" | sed -nr "s/Success! New Package: ([[:alnum:]_]+)/\1/p")

