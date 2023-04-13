# This script both sets up the proposal voting environment and runs all the transaction
# manifests, simulating a full process from start to finish.

# Getting the current script dir
SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )
echo $SCRIPT_DIR

# Resetting resim
resim reset

# Creating some accounts and print details to console
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

# Setting the first account as the default account
resim set-default-account $CHAIRPERSON_ADDRESS $CHAIRPERSON_PRIV_KEY $CHAIRPERSON_OWNER_BADGE

# Publishing the package
PK_OP=$(resim publish ".")
export PACKAGE=$(echo "$PK_OP" | sed -nr "s/Success! New Package: ([[:alnum:]_]+)/\1/p")

# Building the create_component.rtm file as we will need to run it to get the component address
echo "[•] Building the create_component.rtm file"
sed "s/<proposalvoting_package_address>/$PACKAGE/g; s/<chairperson_account_address>/$CHAIRPERSON_ADDRESS/g" $SCRIPT_DIR/raw_transactions/create_component.rtm > $SCRIPT_DIR/transactions/create_component.rtm

# Creating a new proposal voting component
CP_OP=`resim run "$SCRIPT_DIR/transactions/create_component.rtm"`

# Extract and take note of the required addresses
export COMPONENT=$(echo "$CP_OP" | sed -nr "s/.*Component: ([[:alnum:]_]+)/\1/p")
export CHAIRPERSON_BADGE=$(echo "$CP_OP" | sed -nr "s/.*Resource: ([[:alnum:]_]+)/\1/p" | sed '1!d')
export VOTER_BADGE=$(echo "$CP_OP" | sed -nr "s/.*Resource: ([[:alnum:]_]+)/\1/p" | sed '3!d')
export PROPOSAL_RESOURCE=$(echo "$CP_OP" | sed -nr "s/.*Resource: ([[:alnum:]_]+)/\1/p" | sed '4!d')

echo "[•] Component: $COMPONENT"
echo "[•] Chairperson Badge: $CHAIRPERSON_BADGE"
echo "[•] Voter Badge: $VOTER_BADGE"
echo "[•] Proposal Resource: $PROPOSAL_RESOURCE"

# Building a lookup table of all the things to replace:
export REPLACEMENT_LOOKUP=" \
    s/<chairperson_account_address>/$CHAIRPERSON_ADDRESS/g; \
    s/<chairperson_badge_resource_address>/$CHAIRPERSON_BADGE/g; \
    s/<proposalvoting_component_address>/$COMPONENT/g; \
    s/<voting_badge_resource_address>/$VOTER_BADGE/g; \
    s/<voter1_account_address>/$VOTER1_ADDRESS/g; \
    s/<voter2_account_address>/$VOTER2_ADDRESS/g; \
    s/<voter3_account_address>/$VOTER3_ADDRESS/g; \
"

# Building the add_proposals.rtm file"
echo "[•] Building the add_proposals.rtm file"
sed "$REPLACEMENT_LOOKUP" $SCRIPT_DIR/raw_transactions/add_proposals.rtm > $SCRIPT_DIR/transactions/add_proposals.rtm
echo "[•] Running the add_proposals.rtm file to create three proposals"
resim run "$SCRIPT_DIR/transactions/add_proposals.rtm"
# Extract the NonFungibleLocalId's
NF_OP=$(resim show $COMPONENT)
export ALL_PROPOSAL_IDS=$(echo "$NF_OP" | grep -oE '\{[0-9a-f]{8}-[0-9a-f]{4}-4[0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}\}' | sed 's/[{}"]//g')
# Dynamically assign each proposal id
i=1
for id in $ALL_PROPOSAL_IDS; do
    var="PROPOSAL_${i}_ID"
    declare $var="$id"
    i=$((i+1))
done
echo "[•] First proposal id: $PROPOSAL_1_ID"
echo "[•] Second proposal id: $PROPOSAL_2_ID"
echo "[•] Third proposal id: $PROPOSAL_3_ID"

# Add to the replacement lookup table
REPLACEMENT_LOOKUP+=" \
    s/<non_fungible_uuid_1>/$PROPOSAL_1_ID/g; \
    s/<non_fungible_uuid_2>/$PROPOSAL_2_ID/g; \
    s/<non_fungible_uuid_3>/$PROPOSAL_3_ID/g; \
"

# Building the add_voters.rtm file"
echo "[•] Building the add_voters.rtm file"
sed "$REPLACEMENT_LOOKUP" $SCRIPT_DIR/raw_transactions/add_voters.rtm > $SCRIPT_DIR/transactions/add_voters.rtm
echo "[•] Running the add_voters.rtm file to add three voters"
resim run "$SCRIPT_DIR/transactions/add_voters.rtm"

# Building the cast_vote_voter1.rtm file
echo "[•] Building the cast_vote_voter1.rtm file"
sed "$REPLACEMENT_LOOKUP" $SCRIPT_DIR/raw_transactions/cast_vote_voter1.rtm > $SCRIPT_DIR/transactions/cast_vote_voter1.rtm
# Changing to the voter 1 account
resim set-default-account $VOTER1_ADDRESS $VOTER1_PRIV_KEY $VOTER1_OWNER_BADGE
# Running the cast_vote_voter1 manifest
echo "[•] Casting votes as voter 1 using cast_vote_voter1.rtm"
resim run "$SCRIPT_DIR/transactions/cast_vote_voter1.rtm"

# Building the cast_vote_voter2.rtm file
echo "[•] Building the cast_vote_voter2.rtm file"
sed "$REPLACEMENT_LOOKUP" $SCRIPT_DIR/raw_transactions/cast_vote_voter2.rtm > $SCRIPT_DIR/transactions/cast_vote_voter2.rtm
# Changing to the voter 2 account
resim set-default-account $VOTER2_ADDRESS $VOTER2_PRIV_KEY $VOTER2_OWNER_BADGE
# Running the cast_vote_voter2 manifest
echo "[•] Casting votes as voter 2 using cast_vote_voter2.rtm"
resim run "$SCRIPT_DIR/transactions/cast_vote_voter2.rtm"

# Building the cast_vote_voter3.rtm file
echo "[•] Building the cast_vote_voter3.rtm file"
sed "$REPLACEMENT_LOOKUP" $SCRIPT_DIR/raw_transactions/cast_vote_voter3.rtm > $SCRIPT_DIR/transactions/cast_vote_voter3.rtm
# Changing to the voter 3 account
resim set-default-account $VOTER3_ADDRESS $VOTER3_PRIV_KEY $VOTER3_OWNER_BADGE
# Running the cast_vote_voter3 manifest
echo "[•] Casting votes as voter 3 using cast_vote_voter3.rtm"
resim run "$SCRIPT_DIR/transactions/cast_vote_voter3.rtm"

# Changing account back to the chairperson
resim set-default-account $CHAIRPERSON_ADDRESS $CHAIRPERSON_PRIV_KEY $CHAIRPERSON_OWNER_BADGE

# Building the calculate_winning_proposals.rtm file
echo "[•] Building the calculate_winning_proposals.rtm file"
sed "$REPLACEMENT_LOOKUP" $SCRIPT_DIR/raw_transactions/calculate_winning_proposals.rtm > $SCRIPT_DIR/transactions/calculate_winning_proposals.rtm
# Calculating the winning proposals
echo "[•] Calculating winning proposals using the calculate_winning_proposals.rtm file"
resim run "$SCRIPT_DIR/transactions/calculate_winning_proposals.rtm"

# Building the view_results.rtm file
echo "[•] Building the view_results.rtm file"
sed "$REPLACEMENT_LOOKUP" $SCRIPT_DIR/raw_transactions/view_results.rtm > $SCRIPT_DIR/transactions/view_results.rtm
# Viewing the results
echo "[•] Viewing results using the view_results.rtm file"
resim run "$SCRIPT_DIR/transactions/view_results.rtm"