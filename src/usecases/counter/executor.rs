use crate::bindings::{
    WalletProvider,
    blssigcheckoperatorstateretriever::IBLSSignatureCheckerTypes as BlsSigCheckerTypes,
    counter::{BN254, Counter, IBLSSignatureCheckerTypes},
};
use crate::executor::{bls::BlsSignatureVerificationHandler, core::ExecutionResult};
use alloy_primitives::{Bytes, FixedBytes};
use anyhow::Result;
use async_trait::async_trait;

use super::creator::CounterTaskData;

pub struct CounterHandler {
    counter: Counter::CounterInstance<WalletProvider>,
}

impl CounterHandler {
    pub fn new(counter: Counter::CounterInstance<WalletProvider>) -> Self {
        Self { counter }
    }
}

#[async_trait]
impl BlsSignatureVerificationHandler for CounterHandler {
    type TaskData = CounterTaskData;
    async fn handle_verification(
        &mut self,
        msg_hash: FixedBytes<32>,
        quorum_numbers: Bytes,
        current_block_number: u32,
        non_signer_data: BlsSigCheckerTypes::NonSignerStakesAndSignature,
        _task_data: Option<&Self::TaskData>,
    ) -> Result<ExecutionResult> {
        let non_signer_struct_data = IBLSSignatureCheckerTypes::NonSignerStakesAndSignature {
            nonSignerQuorumBitmapIndices: non_signer_data.nonSignerQuorumBitmapIndices,
            nonSignerPubkeys: non_signer_data
                .nonSignerPubkeys
                .into_iter()
                .map(|p| BN254::G1Point { X: p.X, Y: p.Y })
                .collect(),
            quorumApks: non_signer_data
                .quorumApks
                .into_iter()
                .map(|p| BN254::G1Point { X: p.X, Y: p.Y })
                .collect(),
            apkG2: BN254::G2Point {
                X: non_signer_data.apkG2.X,
                Y: non_signer_data.apkG2.Y,
            },
            sigma: BN254::G1Point {
                X: non_signer_data.sigma.X,
                Y: non_signer_data.sigma.Y,
            },
            quorumApkIndices: non_signer_data.quorumApkIndices,
            totalStakeIndices: non_signer_data.totalStakeIndices,
            nonSignerStakeIndices: non_signer_data.nonSignerStakeIndices,
        };

        // Execute the counter increment
        let call_return = self
            .counter
            .increment(
                msg_hash,
                quorum_numbers,
                current_block_number,
                non_signer_struct_data,
            )
            .send()
            .await
            .map_err(|e| anyhow::anyhow!("Failed to send increment transaction: {}", e))?;

        let receipt = call_return
            .get_receipt()
            .await
            .map_err(|e| anyhow::anyhow!("Failed to get transaction receipt: {}", e))?;

        Ok(ExecutionResult {
            transaction_hash: format!("{:?}", receipt.transaction_hash),
            block_number: receipt.block_number,
            gas_used: Some(receipt.gas_used),
            status: Some(receipt.status()),
            contract_address: receipt.contract_address.map(|addr| format!("{:?}", addr)),
        })
    }
}
