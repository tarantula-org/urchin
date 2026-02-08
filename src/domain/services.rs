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

    pub async fn request_action(&self, kind: &str, target: &str, requester: &str, channel: &str, reason: &str) -> anyhow::Result<()> {
        let proposal = BanProposal {
            target_id: target.to_string(),
            reason: reason.to_string(),
            requester_id: requester.to_string(),
            channel_id: channel.to_string(),
            kind: kind.to_string(),
        };

        self.repo.save_proposal(proposal.clone()).await?;

        for notifier in &self.notifiers {
            notifier.notify_proposal(&proposal).await?;
        }
        Ok(())
    }

    pub async fn confirm_action(&self, target: &str, approver: &str) -> anyhow::Result<()> {
        if let Some(proposal) = self.repo.get_proposal(target).await? {
            if proposal.requester_id == approver {
                return Err(anyhow::anyhow!("Self-approval not allowed."));
            }

            for notifier in &self.notifiers {
                notifier.execute_action(&proposal, approver).await?;
            }

            self.repo.delete_proposal(target).await?;
        }
        Ok(())
    }
}