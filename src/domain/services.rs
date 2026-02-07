use std::sync::Arc;
use crate::domain::ports::{BanRepository, PlatformNotifier, BanProposal};

pub struct ConsensusService {
    repo: Arc<dyn BanRepository>,
    notifiers: Vec<Arc<dyn PlatformNotifier>>, 
}

impl ConsensusService {
    pub fn new(repo: Arc<dyn BanRepository>, notifiers: Vec<Arc<dyn PlatformNotifier>>) -> Self {
        Self { repo, notifiers }
    }

    pub async fn request_ban(&self, target: &str, requester: &str) -> anyhow::Result<()> {
        let proposal = BanProposal {
            target_id: target.to_string(),
            reason: "Violation".to_string(),
            requester_id: requester.to_string(),
        };

        self.repo.save_proposal(proposal).await?;

        let message = format!("Ban requested for {} by {}", target, requester);
        for notifier in &self.notifiers {
            notifier.broadcast_alert(&message).await?;
        }
        
        Ok(())
    }
    
    pub async fn check_ban_status(&self, target_id: &str) -> anyhow::Result<()> {
        match self.repo.get_proposal(target_id).await? {
            Some(proposal) => println!("Active proposal: {:?}", proposal),
            None => println!("No active proposal for {}", target_id),
        }
        Ok(())
    }
}

// ... existing code in services.rs ...

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::ports::BanProposal;
    use async_trait::async_trait;
    use std::sync::Mutex;

    // --- Mocks ---

    struct MockRepo;
    #[async_trait]
    impl BanRepository for MockRepo {
        async fn save_proposal(&self, _p: BanProposal) -> anyhow::Result<()> {
            Ok(())
        }
        async fn get_proposal(&self, _id: &str) -> anyhow::Result<Option<BanProposal>> {
            Ok(None)
        }
    }

    struct MockNotifier {
        pub sent: Arc<Mutex<Vec<String>>>,
    }
    #[async_trait]
    impl PlatformNotifier for MockNotifier {
        async fn broadcast_alert(&self, msg: &str) -> anyhow::Result<()> {
            self.sent.lock().unwrap().push(msg.to_string());
            Ok(())
        }
    }

    #[tokio::test]
    async fn test_ban_broadcasts_to_all_platforms() {
        // Arrange
        let sent_msgs = Arc::new(Mutex::new(Vec::new()));
        let notifier = Arc::new(MockNotifier { sent: sent_msgs.clone() });
        let service = ConsensusService::new(
            Arc::new(MockRepo),
            vec![notifier.clone(), notifier.clone()] // Inject twice to simulate 2 platforms
        );

        // Act
        service.request_ban("BadGuy", "Mod").await.unwrap();

        // Assert
        let lock = sent_msgs.lock().unwrap();
        assert_eq!(lock.len(), 2); // Should trigger twice
        assert_eq!(lock[0], "Ban requested for BadGuy by Mod");
    }
}