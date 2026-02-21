use ::std::sync::Arc;
use crate::domain::ports::{BanRepository, PlatformNotifier, BanProposal, Platform};

pub struct ConsensusService {
    repo: Arc<dyn BanRepository>,
    notifiers: ::std::vec::Vec<Arc<dyn PlatformNotifier>>,
}

impl ConsensusService {
    pub fn new(repo: Arc<dyn BanRepository>, notifiers: ::std::vec::Vec<Arc<dyn PlatformNotifier>>) -> Self {
        Self { repo, notifiers }
    }

    pub async fn request_action(
        &self,
        kind: &str,
        target: &str,
        requester: &str,
        origin_platform: Platform,
        origin_channel_id: &str,
        reason: &str,
    ) -> ::anyhow::Result<()> {
        let proposal = BanProposal {
            target_id: target.to_string(),
            reason: reason.to_string(),
            requester_id: requester.to_string(),
            origin_platform,
            origin_channel_id: origin_channel_id.to_string(),
            kind: kind.to_string(),
        };

        self.repo.save_proposal(proposal.clone()).await?;

        for notifier in &self.notifiers {
            let _ = notifier.notify_proposal(&proposal).await;
        }
        Ok(())
    }

    pub async fn confirm_action(&self, target: &str, approver: &str) -> ::anyhow::Result<()> {
        if let Some(proposal) = self.repo.get_proposal(target).await? {
            if proposal.requester_id == approver {
                return Err(::anyhow::anyhow!("Self-approval not allowed."));
            }

            for notifier in &self.notifiers {
                let _ = notifier.execute_action(&proposal, approver).await;
            }

            self.repo.delete_proposal(target).await?;
        }
        Ok(())
    }
}