use ::serde::Deserialize;
use ::std::fs;

#[derive(Debug, Clone, Deserialize)]
pub struct AppConfig {
    pub required_approvals: usize,
    pub command_prefix: ::std::string::String,
    pub expiry_seconds: u64,
}

impl AppConfig {
    pub fn load(path: &str) -> ::anyhow::Result<Self> {
        Ok(::toml::from_str(&fs::read_to_string(path)?)?)
    }
}