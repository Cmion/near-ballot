use near_sdk::log;
use near_sdk::serde_json::json;
use std::collections::HashMap;

pub enum EventTypes {
    CandidateRegistered,
    CandidatesRegistrationStarted,
    CandidateRegistrationFailed,
    VoterRegistered,
    VotersRegistrationStarted,
    VotersRegistrationFailed,
    ElectionStarted,
    ElectionEnded,
    ElectionCancelled,
    CastVote
}

impl EventTypes {
    pub fn to_string(&self) -> String {
        match self {
            EventTypes::CandidateRegistered => String::from("candidate_registered"),
            EventTypes::CandidatesRegistrationStarted => {
                String::from("candidates_registration_started")
            }
            EventTypes::CandidateRegistrationFailed => {
                String::from("candidates_registration_failed")
            }
            EventTypes::VoterRegistered => String::from("voter_registered"),
            EventTypes::VotersRegistrationStarted => String::from("voters_registration_started"),
            EventTypes::VotersRegistrationFailed => String::from("voter_registration_failed"),
            EventTypes::ElectionStarted => String::from("election_started"),
            EventTypes::ElectionEnded => String::from("election_ended"),
            EventTypes::ElectionCancelled => String::from("election_cancelled"),
            EventTypes::CastVote => String::from("cast_vote")
        }
    }
}

pub struct Event {
    pub event_type: EventTypes,
    pub event_data: HashMap<String, String>,
}

impl Event {
    pub fn new(event_type: EventTypes, event_data: HashMap<String, String>) -> Self {
        Event {
            event_type,
            event_data,
        }
    }
    pub fn emit(&self) {
        let event_structure = json!({
            "standard": "nep-297",
            "version": "1.0.0",
            "event": self.event_type.to_string(),
            "data": self.event_data,
        });

        log!(r#"EVENT_JSON:{:?}"#, event_structure.to_string());
    }
}
