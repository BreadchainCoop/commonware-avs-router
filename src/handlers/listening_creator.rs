use NumberEncoder::yourNumbFuncCall;
use alloy::{
    primitives::{Address, U256},
    sol,
    sol_types::SolCall,
};
use alloy_provider::ProviderBuilder;
use alloy_signer_local::PrivateKeySigner;
use std::{env, str::FromStr, sync::Arc};
use tokio::sync::Mutex;
use tracing::info;

use crate::bindings::counter::Counter;
use crate::handlers::{TaskCreator, CounterProvider};
use crate::ingress::{TaskRequest, start_http_server};
use commonware_eigenlayer::config::AvsDeployment;

sol! {
    contract NumberEncoder {
        #[derive(Debug)]
        function yourNumbFunc(uint256 number) public returns (bytes memory);
    }
}

pub struct ListeningCreator {
    counter: Counter::CounterInstance<(), CounterProvider>,
    queue: Arc<Mutex<Vec<TaskRequest>>>,
}

impl ListeningCreator {
    pub fn new(
        provider: CounterProvider,
        counter_address: Address,
    ) -> Self {
        let counter = Counter::new(counter_address, provider.clone());
        Self {
            counter,
            queue: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub async fn get_current_number(&self) -> anyhow::Result<u64> {
        let current_number = self.counter.number().call().await?;
        Ok(current_number._0.to::<u64>())
    }

    pub async fn encode_number_call(&self, number: U256) -> Vec<u8> {
        yourNumbFuncCall { number }.abi_encode()[4..].to_vec()
    }

    // Pulls the next task from the queue, or returns None if empty
    pub async fn get_next_task(&self) -> Option<TaskRequest> {
        let mut queue = self.queue.lock().await;
        if !queue.is_empty() {
            Some(queue.remove(0))
        } else {
            None
        }
    }

    // Single entry point that can be called by the orchestrator
    // This is where queue requests would be pulled from
    pub async fn get_payload_and_round(
        &self,
    ) -> anyhow::Result<(Vec<u8>, u64)> {
        // Wait for a task to be available
        let task = loop {
            if let Some(task) = self.get_next_task().await {
                break task;
            }
            tokio::time::sleep(std::time::Duration::from_millis(100)).await;
        };
        let current_number = self.get_current_number().await?;
        let mut payload = self.get_payload_for_round(current_number).await?.0;
        
        // Encode the three variables into the payload
        payload.extend_from_slice(task.body.var1.as_bytes());
        payload.push(0); // null terminator
        payload.extend_from_slice(task.body.var2.as_bytes());
        payload.push(0); // null terminator
        payload.extend_from_slice(task.body.var3.as_bytes());
        payload.push(0); // null terminator
        
        Ok((payload, current_number))
    }

    // Optional: Method to get payload for a specific round number
    pub async fn get_payload_for_round(
        &self,
        round_number: u64,
    ) -> anyhow::Result<(Vec<u8>, u64)> {
        let encoded = self.encode_number_call(U256::from(round_number)).await;
        info!("Created payload for specific round: {}", round_number);
        Ok((encoded, round_number))
    }

    // Start the HTTP server in a background task
    pub async fn start_http_server(self: Arc<Self>, addr: String) {
        let queue = self.queue.clone();
        tokio::spawn(async move {
            start_http_server(queue, &addr).await;
        });
    }
}

impl TaskCreator for ListeningCreator {
    async fn get_payload_and_round(&self) -> anyhow::Result<(Vec<u8>, u64)> {
        self.get_payload_and_round().await.map_err(|e| anyhow::anyhow!("ListeningCreator error: {}", e))
    }
}

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
    let deployment = AvsDeployment::load().map_err(|e| anyhow::anyhow!("Failed to load deployment: {}", e))?;
    let counter_address = deployment.counter_address().map_err(|e| anyhow::anyhow!("Failed to get counter address: {}", e))?;
    let creator = Arc::new(ListeningCreator::new(provider, counter_address));
    let server_creator = creator.clone();
    tokio::spawn(async move {
        server_creator.start_http_server(addr).await;
    });
    Ok(creator)
}
