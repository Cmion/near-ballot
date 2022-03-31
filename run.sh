#!/bin/bash

clear
source neardev/dev-account.env

#near view "$CONTRACT_NAME" get_election_timeline
near view "$CONTRACT_NAME" get_voters
near view "$CONTRACT_NAME" get_candidates
near view "$CONTRACT_NAME" get_election_results

#near call "$CONTRACT_NAME" register_voter --accountId cd1.cmion.testnet
#near call "$CONTRACT_NAME" register_candidate --accountId cmion2.testnet
#near call "$CONTRACT_NAME" cast_vote '{"candidate": "cmion2.testnet"}' --accountId cd1.cmion.testnet

near view "$CONTRACT_NAME" get_votes '{"from_index": 0, "limit": 10}' --accountId "$CONTRACT_NAME"
