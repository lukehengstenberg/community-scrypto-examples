use scrypto::prelude::*;

/// Represents the metadata inside a proposal NFT.
#[derive(ScryptoSbor, NonFungibleData)]
struct Proposal {
    /// Name of the proposal.
    name: String,
    /// Percentage voting threshold needed for proposal to be funded.
    threshold: u8,
    /// Accumulated number of votes.
    #[mutable]
    votes: u16,
    /// Bool determining if the proposal has won.
    #[mutable]
    won: bool,
}

/// Represents the metadata inside a voter NFT.
#[derive(ScryptoSbor, NonFungibleData)]
struct Voter {
    /// Name of the voter.
    name: String,
    /// Number of votes left to cast.
    #[mutable]
    remaining_votes: u16,
}

#[blueprint]
mod proposal_voting_module {
    /// The ProposalVoting blueprint is a Scrypto blueprint which enables multiple
    /// authenticated parties to vote on proposals, calculating the winning proposals
    /// based on votes received vs a configured percentage threshold. 
    /// NFT's are used to represent and store proposal and voter data.
    struct ProposalVoting {
        /// The internal chairperson badge is used by the ProposalVoting components to
        /// mint, burn, and update the proposal and voter tokens. 
        internal_chairperson_badge: Vault,

        /// When a new proposal is added an updateable NFT is minted to keep track 
        /// of the votes, winning requirements and current status of the proposal. 
        proposal_resource_address: ResourceAddress,

        /// Bool determining if voting is closed.
        is_closed: bool,

        /// Maximum number of votes each person has.
        votes_per_person: u16,

        /// Total number of votes received across all proposals.
        total_votes: u16, 

        /// When a voter is added an NFT is minted to keep track of the votes
        /// they have and to authenticate when they wish to vote on a proposal.
        voter_resource_address: ResourceAddress,

        // Vault storing the proposal NFTs with ResourceAddress proposal_resource_address.
        proposals: Vault,
    }

    impl ProposalVoting {

        /// Creates a new proposal voting component. 
        /// 
        /// This function creates a new ProposalVoting component, assigning the
        /// caller as the "Chair Person" via a Chairperson Badge which is minted
        /// and returned.
        /// Being the ChairPerson means access to authenticated methods for adding 
        /// new proposals, giving people the right to vote, and calculating the 
        /// winning proposals.
        /// (see manifest in transactions/create_component.rtm)
        /// 
        /// # Arguments:
        /// 
        /// * `votes_per_person` (u16) - The number of votes each person has.
        /// 
        /// # Returns:
        /// 
        /// * `ComponentAddress` - The address of the ProposalVoting component.
        /// * `Bucket` - A bucket containing the chairperson badge.
        pub fn instantiate_proposal_voting(
            votes_per_person: u16,
        ) -> (ComponentAddress, Bucket) {
            // Create the chairperson badge.
            let chairperson_badge: Bucket = ResourceBuilder::new_fungible()
                .divisibility(DIVISIBILITY_NONE)
                .metadata("name", "Chairperson Badge")
                .metadata("description", "Used to authenticate the host of the 
                    proposal voting session.")
                .mint_initial_supply(1);

            // Create the proposal component with access rule set to allow chairperson access.
            let proposal_voting_component: ComponentAddress = Self::instantiate_custom_access_proposal_voting(
                votes_per_person,
                rule!(require(chairperson_badge.resource_address())),
            );
            
            return (proposal_voting_component, chairperson_badge);
        }

        /// Creates a new proposal voting component with custom access rules.
        /// 
        /// This function enables creation of a new ProposalVoting component with 
        /// custom access rules for adding proposals, adding voters, and calculating
        /// the winner. This is useful as we may want multiple chair people to have 
        /// access to these methods, rather than a single function caller.
        /// 
        /// # Arguments:
        /// 
        /// * `votes_per_person` (u16) - The number of votes each person has.
        /// * `chairperson_access_rule` (AccessRule) - The access rule for the 
        /// authenticated methods `add_proposal`, `add_voter`, `winning_proposals`.
        /// 
        /// # Returns:
        /// 
        /// * `ComponentAddress` - The address of the ProposalVoting component.
        pub fn instantiate_custom_access_proposal_voting(
            votes_per_person: u16,
            chairperson_access_rule: AccessRule,
        ) -> ComponentAddress {
            // Create the internal chairperson badge to enable minting, burning, and
            // updating of proposal and voter tokens by the ProposalVoting components.
            let internal_chairperson_badge: Bucket = ResourceBuilder::new_fungible()
                .divisibility(DIVISIBILITY_NONE)
                .metadata("name", "Internal Chairperson Badge")
                .metadata("description", "Used for performing internal functionality 
                    of the proposal voting component.")
                .mint_initial_supply(1);

            // Create the voter NFT resource to authenticate voters.
            // Can only be minted, burned, updated, recalled by the internal 
            // chairperson badge.
            let voter_resource_address: ResourceAddress = ResourceBuilder::new_uuid_non_fungible::<Voter>()
                .metadata("name", "Voter Badge")
                .metadata("description", "NFT to authenticate voters.")
                .mintable(
                    rule!(require(internal_chairperson_badge.resource_address())),
                    LOCKED,
                )
                .burnable(
                    rule!(require(internal_chairperson_badge.resource_address())),
                    LOCKED,
                )
                .updateable_non_fungible_data(
                    rule!(require(internal_chairperson_badge.resource_address())),
                    LOCKED,
                )
                .recallable(
                    rule!(require(internal_chairperson_badge.resource_address())),
                    LOCKED,
                )
                .create_with_no_initial_supply();
            
            // Create the proposal NFT resource with mutable supply.
            // Can only be minted, burned, updated by the internal chairperson badge.
            let proposal_resource_address: ResourceAddress = ResourceBuilder::new_uuid_non_fungible::<Proposal>()
                .metadata("name", "Proposals")
                .metadata("description", "The proposals available for voting.")
                .mintable(
                    rule!(require(internal_chairperson_badge.resource_address())),
                    LOCKED,
                )
                .burnable(
                    rule!(require(internal_chairperson_badge.resource_address())),
                    LOCKED,
                )
                .updateable_non_fungible_data(
                    rule!(require(internal_chairperson_badge.resource_address())),
                    LOCKED,
                )
                .create_with_no_initial_supply();

            // Setup the auth rules on the methods to accept access from someone 
            // with a chairperson_badge.
            let access_rules = AccessRulesConfig::new()
                .method("add_proposal", chairperson_access_rule.clone(), LOCKED)
                .method("add_voter", chairperson_access_rule.clone(), LOCKED)
                .method("winning_proposals", chairperson_access_rule.clone(), LOCKED)
                // For other methods we will take the voter badge in a `Proof` to use as auth. 
                .default(rule!(allow_all), LOCKED);

            // Initialize the proposal voting component with our access rules.
            let proposal_voting_component = Self {
                internal_chairperson_badge: Vault::with_bucket(internal_chairperson_badge),
                proposal_resource_address,
                is_closed: false,
                votes_per_person,
                total_votes: 0,
                voter_resource_address,
                proposals: Vault::new(proposal_resource_address),
            }
            .instantiate()
            .globalize_with_access_rules(access_rules);

            // Return the component address.
            return proposal_voting_component;
        }

        /// Adds a new proposal.
        /// 
        /// This method is used to add a new proposal to the ProposalVoting
        /// component with a given name and threshold. New proposal NFTs are 
        /// stored in the proposals vault. 
        /// (see manifest in transactions/add_proposals.rtm)
        /// 
        /// The authorization check is handled on the component level. 
        /// 
        /// # Arguments:
        /// 
        /// * `name` (String) - The name of the proposal.
        /// * `threshold` (u8) - The percentage threshold to win.
        pub fn add_proposal(
            &mut self, 
            name: String, 
            threshold: u8,
        ) {

            info!("Adding proposal {} with a winning threshold of {}%", name, threshold);

            // Populate proposal metadata.
            let proposal: Proposal = Proposal {
                name,
                threshold,
                votes: 0,
                won: false
            };
            // Mint the proposal NFT.
            let proposal_bucket: Bucket = self.internal_chairperson_badge.authorize(|| {
                borrow_resource_manager!(self.proposal_resource_address)
                    .mint_uuid_non_fungible(
                        proposal
                    )
            });
            // Add the new proposal bucket into the proposals vault.
            self.proposals.put(proposal_bucket);
        }

        /// Adds a new voter.
        /// 
        /// This method is used to add a new voter to the ProposalVoting
        /// component with a given name. The minted voter NFT is returned
        /// to the method caller for distribution to a voter account. 
        /// This method is only callable by parties with sufficient access.
        /// The caller is responsible for sending the voter token to the 
        /// correct party. 
        /// (see manifest in transactions/add_voters.rtm)
        /// 
        /// The authorization check is handled on the component level. 
        /// 
        /// # Arguments:
        /// 
        /// * `name` (String) - The name of the voter.
        /// 
        /// # Returns:
        /// 
        /// * `Bucket` - The freshly minted voter NFT.
        pub fn add_voter(
            &mut self, 
            name: String,
        ) -> Bucket {

            info!("Adding a new voter with {} possible votes", self.votes_per_person);

            // Populate voter metadata.
            let voter: Voter = Voter {
                name,
                remaining_votes: self.votes_per_person,
            };
            // Mint the voter NFT.
            let voter_bucket: Bucket = self.internal_chairperson_badge.authorize(|| {
                borrow_resource_manager!(self.voter_resource_address)
                    .mint_uuid_non_fungible(
                        voter
                    )
            });
            
            // Return the voter badge back to the method caller.
            return voter_bucket;
        }

        /// Votes on a given proposal.
        /// 
        /// This method is used to submit votes for a particular proposal. 
        /// It requires proof of a voter badge to be passed in and performs 
        /// several validation checks, before adding votes to the proposal NFT
        /// and removing votes from the voter NFT.
        /// (see manifest in transactions/cast_vote.rtm)
        /// 
        /// # Validation Checks:
        /// 
        /// * **Check 1:** Checks that voting is still open.
        /// * **Check 2:** Checks that the correct type of voter_badge has been provided.
        /// * **Check 3:** Checks that the correct amount of voter_badge has been provided.
        /// * **Check 4:** Checks that the voter has enough remaining votes.
        /// * **Check 5:** Checks that the given proposal id is valid.
        /// 
        /// # Arguments:
        /// 
        /// * `voter_badge` (Proof) - Proof containing the voter NFT for auth and vote counting. 
        /// * `chosen_proposal_id` (NonFungibleLocalId) - Id of the proposal to vote on.
        /// * `number_of_votes` (u16) - Number of votes to cast.
        pub fn vote(
            &mut self,
            voter_badge: Proof,
            chosen_proposal_id: NonFungibleLocalId,
            number_of_votes: u16,
        ) {

            info!("Attempting to submit {} votes for proposal {}", number_of_votes, chosen_proposal_id);

            // Check that voting is still open.
            assert!(
                self.is_closed == false,
                "[Vote]: Voting has closed"
            );
            // Check that the type and quantity of the badge in the proof is correct.
            let voter_badge: ValidatedProof = voter_badge
                .validate_proof(ProofValidationMode::ValidateContainsAmount(
                    self.voter_resource_address,
                    dec!("1"),
                ))
                .expect("[Vote]: Invalid voter badge type or amount present");

            // We have now verified that the caller has presented a valid voter 
            // badge and can extract and update the data.
            let voter_nft: NonFungible<Voter> = voter_badge.non_fungible::<Voter>();
            
            // Retrieve the voter data for the given voter badge.
            let mut voter_data: Voter = voter_nft.data();
            // Check that the voter has sufficient votes left.
            assert!(
                voter_data.remaining_votes >= number_of_votes,
                "[Vote]: Insufficient votes, attempting to cast {} votes but only {} remain", 
                number_of_votes, voter_data.remaining_votes
            );
            // Remove votes being cast.
            voter_data.remaining_votes -= number_of_votes;
            // Extract voter badge id.
            let voter_id = voter_badge.non_fungible_local_id();

            // Update the voter badge NFT data.
            self.internal_chairperson_badge.authorize(|| {
                borrow_resource_manager!(voter_badge.resource_address()).update_non_fungible_data(
                    &voter_id, 
                    "remaining_votes", 
                    voter_data.remaining_votes
                );
            });

            // Check that the chosen proposal exists.
            assert!(
                self.proposals.non_fungible_local_ids().contains(&chosen_proposal_id),
                "[Vote]: Proposal id not recognised"
            );

            // Update the proposal.
            self.internal_chairperson_badge.authorize(|| {
                // Retrieve ResourceManager for the proposal NFT resource.
                let mut proposal_resource_manager: ResourceManager =
                    borrow_resource_manager!(self.proposal_resource_address);
                // Retrieve data for the chosen proposal id.
                let mut proposal: Proposal = 
                    proposal_resource_manager.get_non_fungible_data(&chosen_proposal_id);
                // Increment the number of votes by those cast.
                proposal.votes += number_of_votes;
                // Update the proposal NFT. 
                proposal_resource_manager.update_non_fungible_data(&chosen_proposal_id, 
                    "votes", proposal.votes);
            });

            // Increment the total votes.
            self.total_votes += number_of_votes;
        }

        /// Calculates the winning and losing proposals.
        /// 
        /// This method is used to calculate the winning and losing proposals,
        /// logging the results, updating the proposal NFT's, and closing 
        /// the voting. It should only be called when you are certain voting has
        /// finished. Calculation is done by calculating a percentage based on
        /// the number of votes each proposal has received and the total votes cast.
        /// This is then compared to the percentage threshold established when the
        /// proposal was added.
        /// (see manifest in transactions/calculate_winning_proposals.rtm)
        /// 
        /// # Validation Checks:
        /// 
        /// * **Check 1:** Checks that at least one vote has been cast.
        /// 
        /// The authorization check is handled on the component level. 
        pub fn winning_proposals(
            &mut self,
        ) {
            info!("Calculating winning proposals and closing voting...");

            // Check that at least one vote has been cast.
            assert!(
                self.total_votes > 0,
                "[Calculating_Winner]: No votes have been cast"
            );

            self.internal_chairperson_badge.authorize(|| {
                // Obtain instance of proposal resource manager.
                let mut proposal_resource_manager: ResourceManager =
                    borrow_resource_manager!(self.proposal_resource_address);
                // Loop through all proposal ids to retrieve the corresponding NFT.
                for proposal_id in self.proposals.non_fungible_local_ids() {
                    // Extract proposal data.
                    let proposal_data: Proposal = 
                        proposal_resource_manager.get_non_fungible_data(&proposal_id);
                    // Calculate the percentage of votes received and round to the nearest integer.
                    let percentage_votes_received = 
                        ((proposal_data.votes as f64 * 100.0) / self.total_votes as f64).round() as u8;
                    // Check if the proposal has won.
                    if percentage_votes_received >= proposal_data.threshold {
                        info!("Proposal {} won, receiving {}% of the votes, which 
                            exceeded the minimum threshold of {}%", proposal_data.name, 
                            percentage_votes_received, proposal_data.threshold);
                        // Update the proposal NFT to won = true. 
                        proposal_resource_manager.update_non_fungible_data(&proposal_id, 
                            "won", true);
                    } else {
                        info!("Proposal {} lost, receiving {}% of the votes, which 
                            did not satisfy the minimum threshold of {}%", proposal_data.name,
                            percentage_votes_received, proposal_data.threshold)
                    }
                }
            });

            // Close the voting.
            self.is_closed = true;
            info!("Voting is now closed");
        }

        /// Checks if the given proposal has won.
        /// 
        /// This method can be accessed by anyone to check if a specific proposal
        /// has won. The result is logged for the caller to view.
        /// (see manifest in transactions/view_results.rtm)
        /// 
        /// # Validation Checks:
        /// 
        /// * **Check 1:** Checks that voting has closed.
        /// * **Check 2:** Checks that the given proposal id is valid.
        /// 
        /// # Arguments:
        /// 
        /// * `chosen_proposal_id` (NonFungibleLocalId) - Id of the proposal to 
        /// retrieve the result for.
        pub fn check_winner(
            &mut self,
            chosen_proposal_id: NonFungibleLocalId,
        ) {
            info!("Checking if proposal {} won", chosen_proposal_id);
            // Check that voting is closed.
            assert!(
                self.is_closed == true,
                "[Vote]: Voting is still open"
            );
            // Check that the chosen proposal exists.
            assert!(
                self.proposals.non_fungible_local_ids().contains(&chosen_proposal_id),
                "[Vote]: Proposal id not recognised"
            );

            // Obtain instance of proposal resource manager.
            let proposal_resource_manager: ResourceManager =
                borrow_resource_manager!(self.proposal_resource_address);
            // Extract proposal data.
            let proposal_data: Proposal = 
                proposal_resource_manager.get_non_fungible_data(&chosen_proposal_id);
            // Check and print result.
            if proposal_data.won {
                info!("Proposal {} was successful, exceeding its minimum voting 
                    threshold of {}%", proposal_data.name, proposal_data.threshold);
            } else {
                info!("Proposal {} was not successful, failing to meet its 
                    minimum threshold of {}%", proposal_data.name, proposal_data.threshold);
            }
        }

        /// Logs the result of all proposals.
        /// 
        /// This method can be accessed by anyone to check the results of all
        /// proposals. The method iterates and calls check_winner for each id.
        /// (see manifest in transactions/view_results.rtm)
        pub fn check_results(
            &mut self,
        ) {
            info!("Checking the results for all proposals...");

            // Loop through all proposal ids to check and print the result one at a time.
            for proposal_id in self.proposals.non_fungible_local_ids() {
                self.check_winner(proposal_id);
            }
        }
    }
}