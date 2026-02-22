use ::serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Platform { Discord, Stoat }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Identity {
    pub raw: ::std::string::String,
    pub discord: ::std::option::Option<::std::string::String>,
    pub stoat: ::std::option::Option<::std::string::String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Proposal {
    pub target: Identity,
    pub action: ::std::string::String,
    pub reason: ::std::string::String,
    pub author: ::std::string::String,
    pub origin: Platform,
    pub channel: ::std::string::String,
    pub approvers: ::std::vec::Vec<::std::string::String>,
    pub timestamp: u64,
}

#[derive(Debug)]
pub enum Event {
    Propose { action: ::std::string::String, target: ::std::string::String, author: ::std::string::String, origin: Platform, channel: ::std::string::String, reason: ::std::string::String },
    Approve { target: ::std::string::String, approver: ::std::string::String },
    Cancel { target: ::std::string::String, author: ::std::string::String },
    Sweep,
}