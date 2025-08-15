use crate::ingress::start_http_server;
use crate::usecases::counter::CounterProvider;
use crate::usecases::{
    CreatorConfig, DefaultCreator, DefaultTaskDataFactory, ListeningCreator, SimpleTaskQueue,
};
use alloy_provider::ProviderBuilder;
use alloy_signer_local::PrivateKeySigner;
use commonware_eigenlayer::config::AvsDeployment;
use std::{env, str::FromStr, sync::Arc};

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
) -> anyhow::Result<Arc<ListeningCreator<CounterProvider, DefaultTaskDataFactory, SimpleTaskQueue>>>
{
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
    let creator = Arc::new(ListeningCreator::new(
        state_provider,
        task_data_factory,
        queue.clone(),
        config,
    ));
    let _server_creator = creator.clone();
    let queue = queue.get_queue();
    tokio::spawn(async move {
        start_http_server(queue, &addr).await;
    });
    Ok(creator)
}
