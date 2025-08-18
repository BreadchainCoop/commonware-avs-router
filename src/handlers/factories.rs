use crate::bindings::blsapkregistry::BLSApkRegistry;
use crate::bindings::blssigcheckoperatorstateretriever::BLSSigCheckOperatorStateRetriever;
use crate::bindings::counter::Counter;
use crate::creator::{
    BoxedCreator, CreatorConfig, DefaultCreator, ListeningCreator, SimpleTaskQueue,
};
use crate::executor::contract::ContractExecutor;
use crate::ingress::start_http_server;
use crate::usecases::counter::{
    CounterHandler, CounterProvider, CounterState, DefaultTaskData, DefaultTaskDataFactory,
};
use alloy_provider::ProviderBuilder;
use alloy_signer_local::PrivateKeySigner;
use anyhow::Result;
use commonware_eigenlayer::config::AvsDeployment;
use std::{env, str::FromStr};

/// Factory function to create a default creator
pub async fn create_creator()
-> anyhow::Result<DefaultCreator<CounterProvider, DefaultTaskDataFactory>> {
    let http_rpc = env::var("HTTP_RPC").expect("HTTP_RPC must be set");
    let private_key = env::var("PRIVATE_KEY").expect("PRIVATE_KEY must be set");
    let signer = PrivateKeySigner::from_str(&private_key)
        .map_err(|e| anyhow::anyhow!("Failed to parse private key: {}", e))?;
    let provider = ProviderBuilder::new()
        .wallet(signer)
        .connect(&http_rpc)
        .await
        .map_err(|e| anyhow::anyhow!("Failed to connect provider: {}", e))?;

    let deployment =
        AvsDeployment::load().map_err(|e| anyhow::anyhow!("Failed to load deployment: {}", e))?;
    let counter_address = deployment
        .counter_address()
        .map_err(|e| anyhow::anyhow!("Failed to get counter address: {}", e))?;

    let state_provider = CounterProvider::new(counter_address, provider.clone());
    let task_data_factory = DefaultTaskDataFactory;
    let config = CreatorConfig::default();

    Ok(DefaultCreator::new(
        state_provider,
        task_data_factory,
        config,
    ))
}

/// Factory function to create a listening creator with HTTP server
pub async fn create_listening_creator_with_server(
    addr: String,
) -> anyhow::Result<BoxedCreator<CounterState, DefaultTaskData>> {
    let http_rpc = env::var("HTTP_RPC").expect("HTTP_RPC must be set");
    let private_key = env::var("PRIVATE_KEY").expect("PRIVATE_KEY must be set");
    let signer = PrivateKeySigner::from_str(&private_key)?;
    let provider = ProviderBuilder::new()
        .wallet(signer)
        .connect(&http_rpc)
        .await?;
    let deployment =
        AvsDeployment::load().map_err(|e| anyhow::anyhow!("Failed to load deployment: {}", e))?;
    let counter_address = deployment
        .counter_address()
        .map_err(|e| anyhow::anyhow!("Failed to get counter address: {}", e))?;
    let state_provider = CounterProvider::new(counter_address, provider.clone());
    let task_data_factory = DefaultTaskDataFactory;
    let queue = SimpleTaskQueue::new();
    let config = CreatorConfig::default();
    let creator = Box::new(ListeningCreator::new(
        state_provider,
        task_data_factory,
        queue.clone(),
        config,
    ));
    let queue = queue.get_queue();
    tokio::spawn(async move {
        start_http_server(queue, &addr).await;
    });
    Ok(creator)
}

/// Creates a new ContractExecutor configured for Counter operations
pub async fn create_counter_executor() -> Result<ContractExecutor<CounterHandler>> {
    let http_rpc = env::var("HTTP_RPC").expect("HTTP_RPC must be set");
    let view_only_provider = ProviderBuilder::new().on_http(url::Url::parse(&http_rpc).unwrap());

    let deployment =
        AvsDeployment::load().map_err(|e| anyhow::anyhow!("Failed to load deployment: {}", e))?;
    let bls_apk_registry_address = deployment
        .bls_apk_registry_address()
        .map_err(|e| anyhow::anyhow!("Failed to get BLS APK registry address: {}", e))?;
    let registry_coordinator_address = deployment
        .registry_coordinator_address()
        .map_err(|e| anyhow::anyhow!("Failed to get registry coordinator address: {}", e))?;
    let counter_address = deployment
        .counter_address()
        .map_err(|e| anyhow::anyhow!("Failed to get counter address: {}", e))?;

    let ecdsa_signer =
        PrivateKeySigner::from_str(&env::var("PRIVATE_KEY").expect("PRIVATE_KEY must be set"))
            .map_err(|e| anyhow::anyhow!("Failed to parse private key: {}", e))?;
    let bls_operator_state_retriever_address = deployment
        .bls_sig_check_operator_state_retriever_address()
        .map_err(|e| {
            anyhow::anyhow!("Failed to get BLS operator state retriever address: {}", e)
        })?;

    let write_provider = ProviderBuilder::new()
        .wallet(ecdsa_signer)
        .connect(&http_rpc)
        .await
        .map_err(|e| anyhow::anyhow!("Failed to connect write provider: {}", e))?;

    let bls_apk_registry =
        BLSApkRegistry::new(bls_apk_registry_address, view_only_provider.clone());
    let bls_operator_state_retriever = BLSSigCheckOperatorStateRetriever::new(
        bls_operator_state_retriever_address,
        view_only_provider.clone(),
    );
    let counter = Counter::new(counter_address, write_provider.clone());

    // Create the counter handler
    let counter_handler = CounterHandler::new(counter);

    // Create and return the contract executor
    Ok(ContractExecutor::new(
        view_only_provider,
        bls_apk_registry,
        bls_operator_state_retriever,
        registry_coordinator_address,
        counter_handler,
    ))
}
