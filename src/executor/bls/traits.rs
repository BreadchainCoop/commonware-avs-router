use crate::bindings::blssigcheckoperatorstateretriever::BLSSigCheckOperatorStateRetriever::getNonSignerStakesAndSignatureReturn;
use alloy_primitives::{Bytes, FixedBytes};
use anyhow::Result;
use async_trait::async_trait;

use super::types::BlsVerificationData;

/// BLS-specific executor trait that handles the lower-level BLS signature verification
#[async_trait]
pub trait BlsExecutorTrait: Send + Sync {
    async fn execute_bls_verification(
        &mut self,
        payload_hash: &[u8],
        verification_data: BlsVerificationData,
    ) -> Result<crate::executor::core::ExecutionResult>;
}

/// Contract-specific handler for BLS signature verification in EigenLayer context
#[async_trait]
pub trait BlsSignatureVerificationHandler: Send + Sync {
    async fn handle_verification(
        &mut self,
        msg_hash: FixedBytes<32>,
        quorum_numbers: Bytes,
        current_block_number: u32,
        non_signer_data: getNonSignerStakesAndSignatureReturn,
    ) -> Result<crate::executor::core::ExecutionResult>;
}
