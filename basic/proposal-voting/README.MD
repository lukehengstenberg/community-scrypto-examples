## ProposalVoting
This Scrypto blueprint was created as a means for fairly settling votes on multiple proposals.
This is beneficial for situations where there is only enough funding for a select few proposals so the most popular should be chosen. 
I will be updating this README with some more info tomorrow, for now there should be plenty of detail in my code comments! 

# Todo
1. Add automated tests for complete code coverage in tests/lib.rs.
2. Some code refactoring and cleanup.
3. Expand ./build_rtm.sh and add additional automated scripts for easier setup and usage.
4. Implement extra methods for viewing all results, removing voters and proposals, other useful features.
5. Finish off this README! 

# Features
Coming soon...

# Design Details
Coming soon...

# Steps
Here are the current steps for instantiating and interacting with the `ProposalVoting` blueprint via several transaction manifests.
Currently they require a lot of manual editing to access the correct resources. 
I am planning to add more automation to make life easier when I have time later this week.

1. Run `./build_rtm.sh`, jot down the addresses, private keys, and owner badges as you will need these later to switch accounts.
2. Update values in `create_component.rtm` to match the component address of the faucet, newly added package, and default account address (see comments for locations).
3. Execute `resim run transactions/create_component.rtm` to instantiate the `ProposalVoting` component.
4. Update values in `add_proposals.rtm` to match the default account address and the newly created component address (see comments for locations).
5. Execute `resim run transactions/add_proposals.rtm` to add three new proposals with varying thresholds.
6. Update values in `add_voters.rtm` to match the default account address, the three account addresses, the component address (see comments for locations).
7. Execute `resim run transactions/add_voters.rtm` to add three new voters and deposit a voting badge in each of their accounts.
8. Change to one of the voter accounts so you can cast a vote using `resim set-default-account <COMPONENT_ADDRESS><PRIVATE_KEY><OWNER_BADGE>`.
9. Update values in `cast_vote.rtm` to match the currently selected account, the resource address of the voting badge, the `ProposalVoting` component address, and the id of the proposal to vote on.
10. Repeat steps 8 and 9, switching between accounts/proposals so you have a variety of votes cast.
11. Switch back to the chairperson account using `resim set-default-account <COMPONENT_ADDRESS><PRIVATE_KEY><OWNER_BADGE>`.
12. Update values in `calculate_winning_proposals.rtm` to match the chairperson account and proposal voting component address.
13. Execute `resim run transactions/calculate_winning_proposals.rtm` to calculate the winning proposals based on the proposals added and votes cast.
14. Update values in `view_results.rtm` to match the chairperson account, proposal voting component address, and the desired proposal to check.
15. Execute `resim run transactions/view_results.rtm` to view the result of the given proposal.

# Future work and improvements
Coming soon...
