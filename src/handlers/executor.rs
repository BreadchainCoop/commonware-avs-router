use crate::bindings::blssigcheckoperatorstateretriever::BLSSigCheckOperatorStateRetriever::{self, BLSSigCheckOperatorStateRetrieverInstance};
use crate::bindings::counter::{self, Counter};
use crate::bindings::blsapkregistry::BLSApkRegistry::{self, BLSApkRegistryInstance};
use crate::handlers::{CounterProvider, ViewOnlyProvider};
use alloy::providers::{Provider, ProviderBuilder};
use anyhow::Result;
use alloy::sol_types::SolValue;
use alloy_primitives::{Address, Bytes, FixedBytes, U256};
use alloy_signer_local::PrivateKeySigner;
use bn254::{G1PublicKey, PublicKey, Signature};
use eigen_crypto_bls::convert_to_g1_point;
use std::{env, str::FromStr, collections::HashMap};
use crate::bindings::blssigcheckoperatorstateretriever::BN254::G1Point;
use commonware_utils::hex;
use commonware_eigenlayer::config::AvsDeployment;

pub struct Executor {
    view_only_provider: ViewOnlyProvider,
    bls_apk_registry: BLSApkRegistryInstance<(), ViewOnlyProvider>,
    bls_operator_state_retriever: BLSSigCheckOperatorStateRetrieverInstance<(), ViewOnlyProvider>,
    counter: Counter::CounterInstance<(), CounterProvider>,
    registry_coordinator_address: Address,
    g1_hash_map: HashMap<PublicKey, Address>,
}

impl Executor {
    async fn ensure_g1_hash_map_entry(&mut self, contributor: &PublicKey, g1_pubkey: &G1PublicKey) -> Result<Address> {
        if let Some(address) = self.g1_hash_map.get(contributor) {
            return Ok(*address);
        }

        let g1_point = G1Point {
            X: U256::from_str(&g1_pubkey.get_x()).map_err(|e| anyhow::anyhow!("Failed to parse X coordinate: {}", e))?,
            Y: U256::from_str(&g1_pubkey.get_y()).map_err(|e| anyhow::anyhow!("Failed to parse Y coordinate: {}", e))?,
        };
        let hex_string = format!("0x{}", hex(&alloy_primitives::keccak256(&g1_point.abi_encode().to_vec()).to_vec()));
        let address = self.bls_apk_registry.pubkeyHashToOperator(FixedBytes::<32>::from_str(&hex_string).map_err(|e| anyhow::anyhow!("Failed to parse hex string: {}", e))?).call().await.map_err(|e| anyhow::anyhow!("Failed to get operator from pubkey hash: {}", e))?.operator;
        self.g1_hash_map.insert(contributor.clone(), address);
        Ok(address)
    }

    pub async fn execute_verification(
        &mut self,
        payload_hash: &[u8],
        participating_g1: &[G1PublicKey],
        participating: &[PublicKey],
        signatures: &[Signature],
    ) -> Result<alloy::rpc::types::TransactionReceipt> {
        let (_apk, _apk_g2, asig) =
            bn254::get_points(participating_g1, participating, signatures).ok_or_else(|| anyhow::anyhow!("Failed to get points"))?;
        let asig_g1 = convert_to_g1_point(asig).map_err(|e| anyhow::anyhow!("Failed to convert to G1 point: {}", e))?;
        let sigma_struct = crate::bindings::blssigcheckoperatorstateretriever::BN254::G1Point {
            X: U256::from_str(&asig_g1.X.to_string()).map_err(|e| anyhow::anyhow!("Failed to parse X coordinate: {}", e))?,
            Y: U256::from_str(&asig_g1.Y.to_string()).map_err(|e| anyhow::anyhow!("Failed to parse Y coordinate: {}", e))?,
        };

        let msg_hash = FixedBytes::<32>::from_slice(payload_hash);
        
        // Get or populate operator addresses
        let mut operators = Vec::new();
        for (contributor, g1_pubkey) in participating.iter().zip(participating_g1.iter()) {
            let address = self.ensure_g1_hash_map_entry(contributor, g1_pubkey).await?;
            operators.push(address);
        }

        let current_block_number = self.view_only_provider.get_block_number().await.unwrap();
        let quorum_numbers= Bytes::from_str("0x00").unwrap();
        let ret = self.bls_operator_state_retriever.getNonSignerStakesAndSignature(self.registry_coordinator_address, quorum_numbers.clone(), sigma_struct, operators, current_block_number.try_into().unwrap()).call().await.unwrap()._0;
        let non_signer_struct_data = counter::IBLSSignatureCheckerTypes::NonSignerStakesAndSignature {
            nonSignerQuorumBitmapIndices: ret.nonSignerQuorumBitmapIndices,
            nonSignerPubkeys: ret.nonSignerPubkeys.into_iter().map(|p| counter::BN254::G1Point { X: p.X, Y: p.Y }).collect(),
            quorumApks: ret.quorumApks.into_iter().map(|p| counter::BN254::G1Point { X: p.X, Y: p.Y }).collect(),
            apkG2: counter::BN254::G2Point { X: ret.apkG2.X, Y: ret.apkG2.Y },
            sigma: counter::BN254::G1Point { X: ret.sigma.X, Y: ret.sigma.Y },
            quorumApkIndices: ret.quorumApkIndices,
            totalStakeIndices: ret.totalStakeIndices,
            nonSignerStakeIndices: ret.nonSignerStakeIndices,
        };
        let call_return = self.counter.increment(msg_hash,quorum_numbers,current_block_number.try_into().unwrap(),non_signer_struct_data).send().await.unwrap();
        let receipt = call_return.get_receipt().await.unwrap();
        Ok(receipt)
    }
}

pub async fn create_executor() -> Result<Executor> {
    let http_rpc = env::var("HTTP_RPC").expect("HTTP_RPC must be set");
    let view_only_provider = ProviderBuilder::new()
        .on_http(url::Url::parse(&http_rpc).unwrap());
    
    let deployment = AvsDeployment::load().map_err(|e| anyhow::anyhow!("Failed to load deployment: {}", e))?;
    let bls_apk_registry_address = deployment.bls_apk_registry_address().map_err(|e| anyhow::anyhow!("Failed to get BLS APK registry address: {}", e))?;
    let registry_coordinator_address = deployment.registry_coordinator_address().map_err(|e| anyhow::anyhow!("Failed to get registry coordinator address: {}", e))?;
    let counter_address = deployment.counter_address().map_err(|e| anyhow::anyhow!("Failed to get counter address: {}", e))?;
    
    let ecdsa_signer = PrivateKeySigner::from_str(&env::var("PRIVATE_KEY").expect("PRIVATE_KEY must be set")).map_err(|e| anyhow::anyhow!("Failed to parse private key: {}", e))?;
    let bls_operator_state_retriever_address = deployment.bls_sig_check_operator_state_retriever_address().map_err(|e| anyhow::anyhow!("Failed to get BLS operator state retriever address: {}", e))?;
    
    let write_provider = ProviderBuilder::new()
        .wallet(ecdsa_signer)
        .connect(&http_rpc).await.map_err(|e| anyhow::anyhow!("Failed to connect write provider: {}", e))?;
    let bls_apk_registry = BLSApkRegistry::new(bls_apk_registry_address, view_only_provider.clone());
    let bls_operator_state_retriever = BLSSigCheckOperatorStateRetriever::new(bls_operator_state_retriever_address, view_only_provider.clone());
    let counter = Counter::new(counter_address, write_provider.clone());

    Ok(Executor {
        view_only_provider,
        bls_apk_registry,
        bls_operator_state_retriever,
        counter,
        registry_coordinator_address,
        g1_hash_map: HashMap::new(),
    })
}
