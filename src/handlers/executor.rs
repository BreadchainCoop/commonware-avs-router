use crate::bindings::blsapkregistry::BLSApkRegistry::{self, BLSApkRegistryInstance};
use crate::bindings::blssigcheckoperatorstateretriever::BLSSigCheckOperatorStateRetriever::{
    self, BLSSigCheckOperatorStateRetrieverInstance,
};
use crate::bindings::blssigcheckoperatorstateretriever::BN254::G1Point;
use crate::bindings::counter::{self, Counter};
use alloy::network::EthereumWallet;
use alloy::providers::{Provider, ProviderBuilder};
use alloy::sol_types::SolValue;
use alloy_primitives::{Address, Bytes, FixedBytes, U256};
use alloy_provider::RootProvider;
use alloy_provider::fillers::{
    BlobGasFiller, ChainIdFiller, FillProvider, GasFiller, JoinFill, NonceFiller, WalletFiller,
};
use alloy_signer_local::PrivateKeySigner;
use bn254::{G1PublicKey, PublicKey, Signature};
use commonware_eigenlayer::config::AvsDeployment;
use commonware_utils::hex;
use eigen_crypto_bls::convert_to_g1_point;
use std::{collections::HashMap, env, str::FromStr};

pub struct Executor {
    view_only_provider: FillProvider<
        JoinFill<
            alloy_provider::Identity,
            JoinFill<GasFiller, JoinFill<BlobGasFiller, JoinFill<NonceFiller, ChainIdFiller>>>,
        >,
        RootProvider,
    >,
    bls_apk_registry: BLSApkRegistryInstance<
        (),
        FillProvider<
            JoinFill<
                alloy_provider::Identity,
                JoinFill<GasFiller, JoinFill<BlobGasFiller, JoinFill<NonceFiller, ChainIdFiller>>>,
            >,
            RootProvider,
        >,
    >,
    bls_operator_state_retriever: BLSSigCheckOperatorStateRetrieverInstance<
        (),
        FillProvider<
            JoinFill<
                alloy_provider::Identity,
                JoinFill<GasFiller, JoinFill<BlobGasFiller, JoinFill<NonceFiller, ChainIdFiller>>>,
            >,
            RootProvider,
        >,
    >,
    counter: Counter::CounterInstance<
        (),
        FillProvider<
            JoinFill<
                JoinFill<
                    alloy_provider::Identity,
                    JoinFill<
                        GasFiller,
                        JoinFill<BlobGasFiller, JoinFill<NonceFiller, ChainIdFiller>>,
                    >,
                >,
                WalletFiller<EthereumWallet>,
            >,
            RootProvider,
        >,
    >,
    registry_coordinator_address: Address,
    g1_hash_map: HashMap<PublicKey, Address>,
}

impl Executor {
    async fn ensure_g1_hash_map_entry(
        &mut self,
        contributor: &PublicKey,
        g1_pubkey: &G1PublicKey,
    ) -> Address {
        if let Some(address) = self.g1_hash_map.get(contributor) {
            return *address;
        }

        let g1_point = G1Point {
            X: U256::from_str(&g1_pubkey.get_x()).unwrap(),
            Y: U256::from_str(&g1_pubkey.get_y()).unwrap(),
        };
        let hex_string = format!(
            "0x{}",
            hex(alloy_primitives::keccak256(g1_point.abi_encode()).as_ref())
        );
        let address = self
            .bls_apk_registry
            .pubkeyHashToOperator(FixedBytes::<32>::from_str(&hex_string).unwrap())
            .call()
            .await
            .unwrap()
            .operator;
        self.g1_hash_map.insert(contributor.clone(), address);
        address
    }

    pub async fn execute_verification(
        &mut self,
        payload_hash: &[u8],
        participating_g1: &[G1PublicKey],
        participating: &[PublicKey],
        signatures: &[Signature],
    ) -> Result<alloy::rpc::types::TransactionReceipt, Box<dyn std::error::Error + Send + Sync>>
    {
        let (_apk, _apk_g2, asig) =
            bn254::get_points(participating_g1, participating, signatures).unwrap();
        let asig_g1 = convert_to_g1_point(asig).unwrap();
        let sigma_struct = crate::bindings::blssigcheckoperatorstateretriever::BN254::G1Point {
            X: U256::from_str(&asig_g1.X.to_string()).unwrap(),
            Y: U256::from_str(&asig_g1.Y.to_string()).unwrap(),
        };

        let mut msg_hash_bytes = [0u8; 32];
        if payload_hash.len() >= 32 {
            msg_hash_bytes.copy_from_slice(&payload_hash[0..32]);
        } else {
            msg_hash_bytes[0..payload_hash.len()].copy_from_slice(payload_hash);
        }
        let msg_hash = FixedBytes::<32>::from(msg_hash_bytes);

        // Get or populate operator addresses
        let mut operators = Vec::new();
        for (contributor, g1_pubkey) in participating.iter().zip(participating_g1.iter()) {
            let address = self.ensure_g1_hash_map_entry(contributor, g1_pubkey).await;
            operators.push(address);
        }

        let current_block_number = self.view_only_provider.get_block_number().await.unwrap() - 1;
        let quorum_numbers = Bytes::from_str("0x00").unwrap();
        let ret = self
            .bls_operator_state_retriever
            .getNonSignerStakesAndSignature(
                self.registry_coordinator_address,
                quorum_numbers.clone(),
                sigma_struct,
                operators,
                current_block_number.try_into().unwrap(),
            )
            .call()
            .await
            .unwrap()
            ._0;
        let non_signer_struct_data =
            counter::IBLSSignatureCheckerTypes::NonSignerStakesAndSignature {
                nonSignerQuorumBitmapIndices: ret.nonSignerQuorumBitmapIndices,
                nonSignerPubkeys: ret
                    .nonSignerPubkeys
                    .into_iter()
                    .map(|p| counter::BN254::G1Point { X: p.X, Y: p.Y })
                    .collect(),
                quorumApks: ret
                    .quorumApks
                    .into_iter()
                    .map(|p| counter::BN254::G1Point { X: p.X, Y: p.Y })
                    .collect(),
                apkG2: counter::BN254::G2Point {
                    X: ret.apkG2.X,
                    Y: ret.apkG2.Y,
                },
                sigma: counter::BN254::G1Point {
                    X: ret.sigma.X,
                    Y: ret.sigma.Y,
                },
                quorumApkIndices: ret.quorumApkIndices,
                totalStakeIndices: ret.totalStakeIndices,
                nonSignerStakeIndices: ret.nonSignerStakeIndices,
            };
        let call_return = self
            .counter
            .increment(
                msg_hash,
                quorum_numbers,
                current_block_number.try_into().unwrap(),
                non_signer_struct_data,
            )
            .send()
            .await
            .unwrap();
        let receipt = call_return.get_receipt().await.unwrap();
        Ok(receipt)
    }
}

pub async fn create_executor() -> Result<Executor, Box<dyn std::error::Error + Send + Sync>> {
    let http_rpc = env::var("HTTP_RPC").expect("HTTP_RPC must be set");
    let view_only_provider = ProviderBuilder::new().on_http(url::Url::parse(&http_rpc).unwrap());

    let deployment = AvsDeployment::load()?;
    let bls_apk_registry_address = deployment.bls_apk_registry_address()?;
    let registry_coordinator_address = deployment.registry_coordinator_address()?;
    let counter_address = deployment.counter_address()?;

    let ecdsa_signer =
        PrivateKeySigner::from_str(&env::var("PRIVATE_KEY").expect("PRIVATE_KEY must be set"))
            .unwrap();
    let bls_operator_state_retriever_address =
        deployment.bls_sig_check_operator_state_retriever_address()?;

    let write_provider = ProviderBuilder::new()
        .wallet(ecdsa_signer)
        .connect(&http_rpc)
        .await
        .unwrap();
    let bls_apk_registry =
        BLSApkRegistry::new(bls_apk_registry_address, view_only_provider.clone());
    let bls_operator_state_retriever = BLSSigCheckOperatorStateRetriever::new(
        bls_operator_state_retriever_address,
        view_only_provider.clone(),
    );
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
