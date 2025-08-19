use crate::bindings::blssigcheckoperatorstateretriever::BLSSigCheckOperatorStateRetriever::getNonSignerStakesAndSignatureReturn;
use alloy_primitives::{Bytes, FixedBytes};
use anyhow::Result;
use async_trait::async_trait;
use bn254::{G1PublicKey, PublicKey, Signature};

#[derive(Debug, Clone)]
pub struct ExecutionResult {
    pub transaction_hash: String,
    pub block_number: Option<u64>,
    pub gas_used: Option<u64>,
    pub status: Option<bool>,
    pub contract_address: Option<String>,
}

#[async_trait]
pub trait ExecutorTrait: Send + Sync {
    async fn execute_verification(
        &mut self,
        payload_hash: &[u8],
        participating_g1: &[G1PublicKey],
        participating: &[PublicKey],
        signatures: &[Signature],
    ) -> Result<ExecutionResult>;
}

#[async_trait]
pub trait BlsSignatureVerificationHandler: Send + Sync {
    async fn handle_verification(
        &mut self,
        msg_hash: FixedBytes<32>,
        quorum_numbers: Bytes,
        current_block_number: u32,
        non_signer_data: getNonSignerStakesAndSignatureReturn,
    ) -> Result<ExecutionResult>;
}
