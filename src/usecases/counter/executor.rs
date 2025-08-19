use crate::bindings::WalletProvider;
use crate::bindings::blssigcheckoperatorstateretriever::BLSSigCheckOperatorStateRetriever::getNonSignerStakesAndSignatureReturn;
use crate::bindings::counter::{self, Counter};
use crate::executor::interface::{BlsSignatureVerificationHandler, ExecutionResult};
use alloy_primitives::{Bytes, FixedBytes};
use anyhow::Result;
use async_trait::async_trait;

pub struct CounterHandler {
    counter: Counter::CounterInstance<(), WalletProvider>,
}

impl CounterHandler {
    pub fn new(counter: Counter::CounterInstance<(), WalletProvider>) -> Self {
        Self { counter }
    }

    fn convert_non_signer_data(
        non_signer_data: getNonSignerStakesAndSignatureReturn,
    ) -> counter::IBLSSignatureCheckerTypes::NonSignerStakesAndSignature {
        counter::IBLSSignatureCheckerTypes::NonSignerStakesAndSignature {
            nonSignerQuorumBitmapIndices: non_signer_data._0.nonSignerQuorumBitmapIndices,
            nonSignerPubkeys: non_signer_data
                ._0
                .nonSignerPubkeys
                .into_iter()
                .map(|p| counter::BN254::G1Point { X: p.X, Y: p.Y })
                .collect(),
            quorumApks: non_signer_data
                ._0
                .quorumApks
                .into_iter()
                .map(|p| counter::BN254::G1Point { X: p.X, Y: p.Y })
                .collect(),
            apkG2: counter::BN254::G2Point {
                X: non_signer_data._0.apkG2.X,
                Y: non_signer_data._0.apkG2.Y,
            },
            sigma: counter::BN254::G1Point {
                X: non_signer_data._0.sigma.X,
                Y: non_signer_data._0.sigma.Y,
            },
            quorumApkIndices: non_signer_data._0.quorumApkIndices,
            totalStakeIndices: non_signer_data._0.totalStakeIndices,
            nonSignerStakeIndices: non_signer_data._0.nonSignerStakeIndices,
        }
    }
}

#[async_trait]
impl BlsSignatureVerificationHandler for CounterHandler {
    async fn handle_verification(
        &mut self,
        msg_hash: FixedBytes<32>,
        quorum_numbers: Bytes,
        current_block_number: u32,
        non_signer_data: getNonSignerStakesAndSignatureReturn,
    ) -> Result<ExecutionResult> {
        // Convert to Counter-specific types
        let non_signer_struct_data = Self::convert_non_signer_data(non_signer_data);

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
