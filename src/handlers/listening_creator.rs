use crate::creator::{ListeningCreator as ListeningCreatorImpl, SimpleTaskQueue};
use crate::handlers::counter_provider::CounterProvider;
use crate::ingress::start_http_server;
use alloy_provider::ProviderBuilder;
use alloy_signer_local::PrivateKeySigner;
use std::{env, str::FromStr, sync::Arc};

use commonware_eigenlayer::config::AvsDeployment;

pub type ListeningCreator = ListeningCreatorImpl<CounterProvider, SimpleTaskQueue>;

// Helper function to create a new ListeningCreator instance and start HTTP server
pub async fn create_listening_creator_with_server(
    addr: String,
) -> anyhow::Result<Arc<ListeningCreator>> {
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
    let queue = SimpleTaskQueue::new();
    let creator = Arc::new(ListeningCreatorImpl::new(state_provider, queue));
    let server_creator = creator.clone();
    let queue = server_creator.queue.get_queue();
    tokio::spawn(async move {
        start_http_server(queue, &addr).await;
    });
    Ok(creator)
}
