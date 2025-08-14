use anyhow::Result;

use crate::creator::{base::BaseCreator, interface::TaskCreatorTrait, types::TaskData};

use super::super::base::ContractProviderTrait;

/// Default creator implementation that uses default task data.
///
/// This creator is generic over the contract provider's state type,
/// allowing it to work with any contract provider implementation.
pub struct Creator<P: ContractProviderTrait> {
    base: BaseCreator<P>,
}

impl<P: ContractProviderTrait> Creator<P> {
    pub fn new(contract_provider: P) -> Self {
        let base = BaseCreator::new(contract_provider);
        Self { base }
    }

    async fn get_payload_and_state(&self) -> Result<(Vec<u8>, P::State)> {
        let current_state = self.get_current_state().await?;
        let encoded_state = self.base.encode_state_call(&current_state).await;
        let task_data = self.get_task_data().await?;
        let payload = task_data.encode_into_payload(encoded_state);
        Ok((payload, current_state))
    }

    async fn get_task_data(&self) -> Result<TaskData> {
        Ok(TaskData::default())
    }
}

#[async_trait::async_trait]
impl<P: ContractProviderTrait> TaskCreatorTrait<P::State> for Creator<P> {
    async fn get_payload_and_state(&self) -> anyhow::Result<(Vec<u8>, P::State)> {
        self.get_payload_and_state()
            .await
            .map_err(|e| anyhow::anyhow!("Creator error: {}", e))
    }

    async fn get_current_state(&self) -> anyhow::Result<P::State> {
        self.base.get_current_state().await
    }
}
