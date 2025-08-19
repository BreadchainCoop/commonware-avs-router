use anyhow::Result;
use async_trait::async_trait;

use super::types::VerificationData;

#[async_trait]
pub trait VerificationExecutor: Send + Sync {
    async fn execute_verification(
        &mut self,
        payload_hash: &[u8],
        verification_data: VerificationData,
    ) -> Result<ExecutionResult>;
}

#[derive(Debug, Clone)]
pub struct ExecutionResult {
    pub transaction_hash: String,
    pub block_number: Option<u64>,
    pub gas_used: Option<u64>,
    pub status: Option<bool>,
    pub contract_address: Option<String>,
}
