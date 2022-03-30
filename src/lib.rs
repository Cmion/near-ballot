pub mod duration;

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{UnorderedMap, UnorderedSet};
use near_sdk::{env, near_bindgen, AccountId, BorshStorageKey};

use crate::duration::*;

near_sdk::setup_alloc!();

// 5 Ⓝ in yoctoNEAR
// const PRIZE_AMOUNT: u128 = 5_000_000_000_000_000_000_000_000;
const ELECTION_DURATION: u64 = 3 * 60 * 1000 * 1_000_000; // 30 days
const CANDIDATE_REGISTRATION_DURATION: u64 = 3 * 60 * 1000 * 1_000_000; // 5 days
const VOTER_REGISTRATION_DURATION: u64 = 3 * 60 * 1000 * 1_000_000; // 5 days

#[derive(BorshSerialize, BorshStorageKey)]
pub enum StorageKeys {
    Candidate,
    Voter,
    Votes,
}
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Ballot {
    pub voters: UnorderedSet<String>,
    pub votes: UnorderedMap<String, String>,
    pub candidates: UnorderedSet<String>,
    pub election_duration: BallotDuration,
    pub candidate_registration_duration: BallotDuration,
    pub voter_registration_duration: BallotDuration,
}

impl Default for Ballot {
    fn default() -> Self {
        let block_timestamp: u64 = env::block_timestamp();
        let candidate_registration_duration = BallotDuration::new(
            block_timestamp,
            block_timestamp + CANDIDATE_REGISTRATION_DURATION,
            Some(String::from("Candidates")),
        );
        let voter_registration_duration = BallotDuration::new(
            candidate_registration_duration.end(),
            candidate_registration_duration.end() + VOTER_REGISTRATION_DURATION,
            Some(String::from("Voters")),
        );
        let election_duration = BallotDuration::new(
            voter_registration_duration.end(),
            voter_registration_duration.end() + ELECTION_DURATION,
            Some(String::from("Election")),
        );
        Self {
            voters: UnorderedSet::new(StorageKeys::Voter),
            votes: UnorderedMap::new(StorageKeys::Votes),
            candidates: UnorderedSet::new(StorageKeys::Candidate),
            candidate_registration_duration,
            voter_registration_duration,
            election_duration,
        }
    }
}

#[near_bindgen]
impl Ballot {
    pub fn get_election_timeline(self) -> (BallotDuration, BallotDuration, BallotDuration) {
        (
            self.candidate_registration_duration,
            self.voter_registration_duration,
            self.election_duration,
        )
    }

    pub fn get_candidates(&self) -> Vec<String> {
        self.candidates.iter().map(|v| v.to_string()).collect()
    }

    pub fn get_voters(&self) -> Vec<String> {
        self.voters.iter().map(|v| v.to_string()).collect()
    }

    pub fn get_votes(&self, from_index: u64, limit: u64) -> Vec<(AccountId, String)> {
        let keys = self.votes.keys_as_vector();
        let values = self.votes.values_as_vector();

        // Gets a slice from the index to the minimum of the index + limit or the length of the vector
        (from_index..std::cmp::min(from_index + limit, keys.len()))
            .map(|index| (keys.get(index).unwrap(), values.get(index).unwrap()))
            .collect()
    }

    pub fn get_vote(&self, voter: &AccountId) -> Option<AccountId> {
        self.votes.get(voter)
    }

    pub fn get_candidate_votes_count(&self, candidate: &AccountId) -> u64 {
        let values = self.votes.to_vec();

        values
            .iter()
            .filter(|(key, _value)| **key == *candidate)
            .count() as u64
    }

    pub fn voter_has_registered(&self, voter: &AccountId) -> bool {
        self.voters.contains(voter)
    }

    pub fn voter_has_voted(&self, voter: &AccountId) -> bool {
        self.votes.get(voter).is_some()
    }

    pub fn candidate_has_registered(&self, candidate: &AccountId) -> bool {
        self.candidates.contains(candidate)
    }

    // #[payable]
    pub fn register_candidate(&mut self) {
        if self.candidate_registration_duration.is_in_future() {
            env::panic(b"Candidate registration period has not started");
        }

        if self.candidate_registration_duration.is_expired() {
            env::panic(b"Candidate registration period has ended");
        }
        // if near_sdk::env::attached_deposit() == 0 {
        //     near_sdk::env::panic(b"You need to pay to register a candidate");
        // }
        let candidate = env::predecessor_account_id();
        if !self.candidates.contains(&candidate) {
            self.candidates.insert(&candidate);
            env::log(
                format!("You successfully registered as a candidate: {}", candidate).as_bytes(),
            )
        } else {
            env::panic("Candidate already registered".as_bytes());
        }
    }

    pub fn register_voter(&mut self) {
        if self.voter_registration_duration.is_in_future() {
            env::panic(b"Voter registration period has not started");
        }
        if self.voter_registration_duration.is_expired() {
            env::panic(b"Voter registration period has ended");
        }
        let voter = env::predecessor_account_id();
        if !self.voters.contains(&voter) {
            self.voters.insert(&voter);
            env::log(format!("You have successfully registered as |{}| to vote", voter).as_bytes())
        } else {
            env::panic("Voter already registered".as_bytes());
        }
    }

    pub fn cast_vote(&mut self, candidate: AccountId) {
        if self.election_duration.is_in_future() {
            env::panic(b"Voting has not started");
        }
        if self.election_duration.is_expired() {
            env::panic(b"Election period has ended");
        }
        let voter = env::predecessor_account_id();
        if !self.candidate_has_registered(&candidate) {
            env::panic("Unrecognised candidate".as_bytes());
        }

        if !self.voter_has_registered(&voter) {
            env::panic("You have not registered".as_bytes());
        }

        if self.voter_has_voted(&voter) {
            env::panic("You already voted".as_bytes());
        }

        self.voters.insert(&voter);
        self.votes.insert(&voter, &candidate);

        env::log(format!("{} voted for {}", voter, candidate).as_bytes());
    }
}
