use crate::domain::{models::Proposal, ports::StateStore};
use ::anyhow::Result;

pub struct SledStore(::sled::Db);

impl SledStore {
    pub fn new(path: &str) -> Result<Self> {
        Ok(Self(::sled::Config::new().path(path).cache_capacity(64_000_000).open()?))
    }
}

#[::async_trait::async_trait]
impl StateStore for SledStore {
    async fn save(&self, p: Proposal) -> Result<()> {
        self.0.insert(&p.target.raw, ::serde_json::to_vec(&p)?)?;
        self.0.flush_async().await?;
        Ok(())
    }
    
    async fn get(&self, target: &str) -> Result<::std::option::Option<Proposal>> {
        if let Some(v) = self.0.get(target)? {
            match ::serde_json::from_slice(&v) {
                Ok(p) => return Ok(Some(p)),
                Err(_) => {
                    let _ = self.0.remove(target);
                }
            }
        }
        Ok(None)
    }
    
    async fn remove(&self, target: &str) -> Result<()> {
        self.0.remove(target)?;
        self.0.flush_async().await?;
        Ok(())
    }
    
    async fn list(&self) -> Result<::std::vec::Vec<Proposal>> {
        let mut res = ::std::vec::Vec::new();
        for r in self.0.iter() {
            if let Ok((k, v)) = r {
                match ::serde_json::from_slice(&v) {
                    Ok(p) => res.push(p),
                    Err(_) => {
                        let _ = self.0.remove(&k);
                    }
                }
            }
        }
        Ok(res)
    }
}