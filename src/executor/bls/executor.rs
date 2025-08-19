use crate::bindings::ReadOnlyProvider;
use crate::bindings::blsapkregistry::BLSApkRegistry::BLSApkRegistryInstance;
use crate::bindings::blssigcheckoperatorstateretriever::BN254::G1Point;
use crate::bindings::blssigcheckoperatorstateretriever::{
    BLSSigCheckOperatorStateRetriever::BLSSigCheckOperatorStateRetrieverInstance,
    BLSSigCheckOperatorStateRetriever::getNonSignerStakesAndSignatureReturn,
};
use crate::executor::core::{ExecutionResult, VerificationData, VerificationExecutor};
use alloy::providers::Provider;
use alloy::sol_types::SolValue;
use alloy_primitives::{Address, Bytes, FixedBytes, U256};
use anyhow::Result;
use async_trait::async_trait;
use bn254::{G1PublicKey, PublicKey};
use commonware_utils::hex;
use eigen_crypto_bls::convert_to_g1_point;
use std::{collections::HashMap, str::FromStr};
use tracing::debug;

use super::traits::{BlsExecutorTrait, BlsSignatureVerificationHandler};
use super::types::BlsVerificationData;

pub struct BlsEigenlayerExecutor<H: BlsSignatureVerificationHandler> {
    view_only_provider: ReadOnlyProvider,
    bls_apk_registry: BLSApkRegistryInstance<(), ReadOnlyProvider>,
    bls_operator_state_retriever: BLSSigCheckOperatorStateRetrieverInstance<(), ReadOnlyProvider>,
    registry_coordinator_address: Address,
    contract_handler: H,
    g1_hash_map: HashMap<PublicKey, Address>,
}

impl<H: BlsSignatureVerificationHandler> BlsEigenlayerExecutor<H> {
    pub fn new(
        view_only_provider: ReadOnlyProvider,
        bls_apk_registry: BLSApkRegistryInstance<(), ReadOnlyProvider>,
        bls_operator_state_retriever: BLSSigCheckOperatorStateRetrieverInstance<
            (),
            ReadOnlyProvider,
        >,
        registry_coordinator_address: Address,
        contract_handler: H,
    ) -> Self {
        Self {
            view_only_provider,
            bls_apk_registry,
            bls_operator_state_retriever,
            registry_coordinator_address,
            contract_handler,
            g1_hash_map: HashMap::new(),
        }
    }

    async fn ensure_g1_hash_map_entry(
        &mut self,
        contributor: &PublicKey,
        g1_pubkey: &G1PublicKey,
    ) -> Result<Address> {
        if let Some(address) = self.g1_hash_map.get(contributor) {
            return Ok(*address);
        }

        let g1_point = G1Point {
            X: U256::from_str(&g1_pubkey.get_x())
                .map_err(|e| anyhow::anyhow!("Failed to parse X coordinate: {}", e))?,
            Y: U256::from_str(&g1_pubkey.get_y())
                .map_err(|e| anyhow::anyhow!("Failed to parse Y coordinate: {}", e))?,
        };
        let hex_string = format!(
            "0x{}",
            hex(alloy_primitives::keccak256(g1_point.abi_encode()).as_ref())
        );
        let address = self
            .bls_apk_registry
            .pubkeyHashToOperator(
                FixedBytes::<32>::from_str(&hex_string)
                    .map_err(|e| anyhow::anyhow!("Failed to parse hex string: {}", e))?,
            )
            .call()
            .await
            .map_err(|e| anyhow::anyhow!("Failed to get operator from pubkey hash: {}", e))?
            .operator;
        self.g1_hash_map.insert(contributor.clone(), address);
        Ok(address)
    }
}

#[async_trait]
impl<H: BlsSignatureVerificationHandler> VerificationExecutor for BlsEigenlayerExecutor<H> {
    async fn execute_verification(
        &mut self,
        payload_hash: &[u8],
        verification_data: VerificationData,
    ) -> Result<ExecutionResult> {
        // Convert generic verification data to BLS-specific data
        // For BLS, we need to extract G1 public keys from the context
        let g1_public_keys = if let Some(_context) = verification_data.context {
            // In a real implementation, you'd deserialize the G1 public keys from context
            // For now, we'll assume they're provided in the same order as public_keys
            // This is a simplified approach - in practice you'd want proper serialization
            vec![] // TODO: Implement proper G1 public key extraction from context
        } else {
            return Err(anyhow::anyhow!(
                "BLS verification requires G1 public keys in context"
            ));
        };

        let bls_verification_data = BlsVerificationData::new(
            verification_data.signatures,
            verification_data.public_keys,
            g1_public_keys,
        );

        self.execute_bls_verification(payload_hash, bls_verification_data)
            .await
    }
}

#[async_trait]
impl<H: BlsSignatureVerificationHandler> BlsExecutorTrait for BlsEigenlayerExecutor<H> {
    async fn execute_bls_verification(
        &mut self,
        payload_hash: &[u8],
        verification_data: BlsVerificationData,
    ) -> Result<ExecutionResult> {
        let participating_g1 = &verification_data.g1_public_keys;
        let participating = &verification_data.public_keys;
        let signatures = &verification_data.signatures;
        let (_apk, _apk_g2, asig) = bn254::get_points(participating_g1, participating, signatures)
            .ok_or_else(|| anyhow::anyhow!("Failed to get points"))?;
        let asig_g1 = convert_to_g1_point(asig)
            .map_err(|e| anyhow::anyhow!("Failed to convert to G1 point: {}", e))?;
        let sigma_struct = crate::bindings::blssigcheckoperatorstateretriever::BN254::G1Point {
            X: U256::from_str(&asig_g1.X.to_string())
                .map_err(|e| anyhow::anyhow!("Failed to parse X coordinate: {}", e))?,
            Y: U256::from_str(&asig_g1.Y.to_string())
                .map_err(|e| anyhow::anyhow!("Failed to parse Y coordinate: {}", e))?,
        };

        let msg_hash = FixedBytes::<32>::from_slice(payload_hash);

        // Get or populate operator addresses
        let mut operators = Vec::new();
        for (contributor, g1_pubkey) in participating.iter().zip(participating_g1.iter()) {
            let address = self
                .ensure_g1_hash_map_entry(contributor, g1_pubkey)
                .await?;
            operators.push(address);
        }

        let current_block_number = self
            .view_only_provider
            .get_block_number()
            .await
            .map_err(|e| anyhow::anyhow!("Failed to get block number: {}", e))?;
        let quorum_numbers = Bytes::from_str("0x00")
            .map_err(|e| anyhow::anyhow!("Failed to parse quorum numbers: {}", e))?;
        let ret = self
            .bls_operator_state_retriever
            .getNonSignerStakesAndSignature(
                self.registry_coordinator_address,
                quorum_numbers.clone(),
                sigma_struct,
                operators,
                current_block_number
                    .try_into()
                    .map_err(|e| anyhow::anyhow!("Failed to convert block number: {}", e))?,
            )
            .call()
            .await
            .map_err(|e| anyhow::anyhow!("Failed to get non-signer stakes and signature: {}", e))?
            ._0;

        // Pass the non-signer data directly to the contract handler
        let non_signer_return = getNonSignerStakesAndSignatureReturn { _0: ret };

        // Delegate the contract-specific execution to the handler
        let result = self
            .contract_handler
            .handle_verification(
                msg_hash,
                quorum_numbers,
                current_block_number
                    .try_into()
                    .map_err(|e| anyhow::anyhow!("Failed to convert block number: {}", e))?,
                non_signer_return,
            )
            .await?;

        debug!(
            transaction_hash = %result.transaction_hash,
            block_number = ?result.block_number,
            gas_used = ?result.gas_used,
            status = ?result.status,
            contract_address = ?result.contract_address,
            "Contract execution result"
        );

        Ok(result)
    }
}
