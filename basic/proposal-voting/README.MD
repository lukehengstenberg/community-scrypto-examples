# ProposalVoting
This Scrypto blueprint was created as a means for fairly settling votes on proposals.
This is beneficial for situations where there is only enough funding for a select few proposals so the most popular should be chosen.

For example, say a government department only has enough funding for a new playground, a waterslide, and a car park. Using the ProposalVoting component, each of these could be launched as its own proposal NFT with percentage thresholds for success. The chairperson(s) could then mint voter badges, each containing a set number of votes, and distribute them to the other committee members. Each committee member can now use their voter badge to allocate their votes to the proposal(s) they think are most important. Every time they cast a vote, the balance of the proposal NFT increases and the balance of their voter badge decreases until they are out of votes. Finally, the chairperson(s) can decide to close voting, calculating the winning proposals based on whether the percentage of votes received has exceeded the minimum percentage threshold, and updating the proposal NFT to reflect the result. 
The government department now has non-fungible evidence to back their decision to get a new slide or swing!

## Features
The `ProposalVoting` blueprint comes with a number of features, including:
1. Ability to launch the contract with protection over who can add proposals, add voters, and calculate results. 
2. Ability to allow multiple parties to access authorized methods through support for custom access rules. 
3. Restrictions on voting to only parties that can show proof of a voter badge, validation checks to ensure voting only happens before it is closed.
4. Methods for adding new proposals, voters, voting, and calculating/displaying results. 

## Steps
There are two automated scripts available for instantiating and interacting with the `ProposalVoting` blueprint via several manifests:
- `build_rtm.sh`: This script will launch the blueprint component, adding three proposals and three voters. All the transaction manifests should be auto-populated. The user can then do the following to run through the process:
1. Run `./build_rtm.sh`, jot down the addresses, private keys, and owner badges towards the top of the console output as you will need these later to switch accounts.
2. Change to one of the voter accounts so you can cast a vote using `resim set-default-account <COMPONENT_ADDRESS><PRIVATE_KEY><OWNER_BADGE>`.
3. Execute the corresponding transaction manifest e.g. `resim run transactions/cast_vote_voter1.rtm` if you are set to voter 1. OR add your own parameters into `cast_vote.rtm` and run that.
4. Repeat steps 2 and 3, switching between accounts/proposals so you have a variety of votes cast.
5. Switch back to the chairperson account using `resim set-default-account <COMPONENT_ADDRESS><PRIVATE_KEY><OWNER_BADGE>`.
6. Execute `resim run transactions/calculate_winning_proposals.rtm` to calculate the winning proposals based on the proposals added and votes cast.
7. Execute `resim run transactions/view_results.rtm` to view the results.

- `build_run_rtm.sh`: This script will complete the full process, adding three proposals and three voters, casting a variety of votes, calculating and outputting the results. Simply run the script using `./build_run_rtm.sh`.

## Future work and improvements
There are several improvements that could be made, here are some suggestions of features that may be added:
- [ ] Automated Tests for all methods including edge-cases. 
- [x] Better setup/execution automated scripts that can auto-populate addresses in transaction manifests. 
- [ ] Ability to set a time-limit for how long voting is open, and automatically closing and calculating results when elapsed.
- [ ] Bulk minting voter badges for better efficiency in situations where we need lots of participation. 
- [ ] Bulk minting proposals, again for better efficiency. 
- [ ] Methods for chairperson to remove/recall proposals and voters as in reality proposals/voters could be dropped.
- [x] Method for bulk printing/viewing results. 
- [ ] User interface which makes use of transaction manifests to interact with the `ProposalVoting` component.
- [ ] More proposal metadata e.g. detailed description of what the proposal is. 
- [ ] Additional transaction manifests for simulating extra scenarios like instantiation with custom access rules.

