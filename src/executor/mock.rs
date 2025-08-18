use crate::bindings::blssigcheckoperatorstateretriever::BLSSigCheckOperatorStateRetriever::getNonSignerStakesAndSignatureReturn;
use crate::executor::interface::{ContractHandler, ExecutionResult};
use alloy_primitives::{Bytes, FixedBytes};
use anyhow::Result;
use async_trait::async_trait;

/// Mock implementation of ContractHandler for testing purposes.
pub struct MockContractHandler;

impl MockContractHandler {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl ContractHandler for MockContractHandler {
    async fn handle_verification(
        &mut self,
        _msg_hash: FixedBytes<32>,
        _quorum_numbers: Bytes,
        _current_block_number: u32,
        _non_signer_data: getNonSignerStakesAndSignatureReturn,
    ) -> Result<ExecutionResult> {
        // Mock implementation returns success with dummy values
        Ok(ExecutionResult {
            transaction_hash: "0x1234567890abcdef".to_string(),
            block_number: Some(12345),
            gas_used: Some(21000),
        })
    }
}
