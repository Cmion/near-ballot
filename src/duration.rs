use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::env;
use near_sdk::serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct BallotDuration {
    pub start: u64,
    pub end: u64,
    pub name: String,
}

impl BallotDuration {
    pub fn new(start: u64, end: u64, name: Option<String>) -> Self {
        Self {
            start,
            end,
            name: name.unwrap_or(String::from("Duration")),
        }
    }

    pub fn start(&self) -> u64 {
        self.start
    }

    pub fn end(&self) -> u64 {
        self.end
    }

    pub fn get_duration(&self) -> u64 {
        self.end - self.start
    }

    pub fn get(self) -> BallotDuration {
        self
    }

    pub fn to_millis(&self) -> Self {
        let micros = self.to_micros();
        Self {
            start: micros.start / (1000),
            end: micros.end / (1000),
            name: String::from(&self.name),
        }
    }

    pub fn to_micros(&self) -> Self {
        Self {
            start: self.start / 1_000,
            end: self.end / 1_000,
            name: String::from(&self.name),
        }
    }

    pub fn to_seconds(&self) -> Self {
        let millis = self.to_millis();
        Self {
            start: millis.start / 1_000,
            end: millis.end / 1_000,
            name: String::from(&self.name),
        }
    }

    pub fn is_active(&self) -> bool {
        let now = env::block_timestamp();
        self.start <= now && now < self.end
    }

    pub fn is_expired(&self) -> bool {
        let now = env::block_timestamp();
        self.end <= now
    }
    pub fn is_in_future(&self) -> bool {
        let now = env::block_timestamp();
        self.start > now
    }
}
