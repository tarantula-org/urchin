use async_trait::async_trait;
use sled::Db;
use serde_json;
use crate::domain::ports::{BanRepository, BanProposal};

pub struct SledRepository {
    tree: Db,
}

impl SledRepository {
    pub fn new(path: &str) -> anyhow::Result<Self> {
        let db = sled::open(path)?;
        Ok(Self { tree: db })
    }
}

#[async_trait]
impl BanRepository for SledRepository {
    async fn save_proposal(&self, proposal: BanProposal) -> anyhow::Result<()> {
        let key = proposal.target_id.as_bytes();
        let value = serde_json::to_vec(&proposal)?;
        
        self.tree.insert(key, value)?;
        self.tree.flush_async().await?;
        
        Ok(())
    }

    async fn get_proposal(&self, target_id: &str) -> anyhow::Result<Option<BanProposal>> {
        let key = target_id.as_bytes();
        
        if let Some(ivec) = self.tree.get(key)? {
            let proposal: BanProposal = serde_json::from_slice(&ivec)?;
            Ok(Some(proposal))
        } else {
            Ok(None)
        }
    }

    async fn delete_proposal(&self, target_id: &str) -> anyhow::Result<()> {
        let key = target_id.as_bytes();
        self.tree.remove(key)?;
        self.tree.flush_async().await?;
        Ok(())
    }
}