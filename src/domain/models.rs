use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Platform { Discord, Stoat }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Identity {
    pub raw: String,
    pub discord: Option<String>,
    pub stoat: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Proposal {
    pub target: Identity,
    pub action: String,
    pub reason: String,
    pub author: String,
    pub origin: Platform,
    pub channel: String,
}

#[derive(Debug)]
pub enum Event {
    Propose { action: String, target: String, author: String, origin: Platform, channel: String, reason: String },
    Approve { target: String, approver: String },
}