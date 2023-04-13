# TrustMore

## Project Outline

Today's philosophy with regards to dApps and smart-contracts seems to be trust-less. But use-cases that can not be covered by smart-code are easy to imagine. A contract between two parties that both contribute to an escrow vault has a risk of locking the tokens in case of a conflict. A proven method to resolve conflicts is the use of a Mediator or Notary.</br></br>

In this project a Mediator dApp and Contract dApp are made. A great care has been taken to make sure the Mediator and Contract dApp are secured from malicious use.</br></br>

Both Packages are setup in a way that <strong>ONLY</strong> the holder of the owner-token used to publish the blueprint is able to instantiate a component from the blueprint.</br></br>

After publishing the Contract and Mediator dApp, this owner-token of the contract blueprint is transferred to the Mediator dApp; this ends the Validated Contract setup.</br></br>

A (paid for) validated Contract can be requested by any party from the Mediator dApp.
This instantiation will generate a Buyer, Seller and Mediator token. The requestor can distribute the buyer and seller token to the correct parties, while the mediator token is automatically stored in the Mediator component. This guarantees the Mediator is alway a part of an instatiated contract.</br></br>

The Buyer and Seller token have to be activated, a task that can only be done by the token holder. If an already activated token has been provided, one must assume the contract has been tampered with and a new contract should be generated.</br></br>

The actual code for the workings of the contract is not part of this code-example.</br></br>

## Getting Started
-   Source the sourceme on Linux/Bash for an easy start.

        %-> source sourceme
