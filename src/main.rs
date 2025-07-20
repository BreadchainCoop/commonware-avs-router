//! Aggregate signatures from multiple contributors over the BN254 curve.
//!
//! # Usage (3 of 4 Threshold)
mod bindings;
mod handlers;

use ark_bn254::{Fr};
use bn254::{Bn254, PrivateKey};
use clap::{Arg, Command};
use commonware_cryptography::Signer;
use commonware_p2p::authenticated::lookup::{self, Network};
use commonware_runtime::{
    Metrics, Runner, Spawner,
    tokio::{self},
};
use governor::Quota;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::{
    net::{IpAddr, Ipv4Addr, SocketAddr},
    num::NonZeroU32,
};
use commonware_utils::NZU32;
use std::str::FromStr;
use commonware_eigenlayer::network_configuration::{EigenStakingClient, QuorumInfo};
use std::env;
use dotenv;
use eigen_logging::log_level::LogLevel;

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
struct KeyConfig {
    privateKey: String,
}

fn load_key_from_file(path: &str) -> String {
    let contents = fs::read_to_string(path).expect("Could not read key file");
    let config: KeyConfig = serde_json::from_str(&contents).expect("Could not parse key file");
    config.privateKey
}

fn get_signer_from_fr(key: &str) -> Bn254 {
    let fr = Fr::from_str(key).expect("Invalid decimal string for private key");
    let key = PrivateKey::from(fr);
    <Bn254 as Signer>::from(key).expect("Failed to create signer")
}

// Unique namespace to avoid message replay attacks.
const APPLICATION_NAMESPACE: &[u8] = b"_COMMONWARE_AGGREGATION_";

fn configure_identity(matches: &clap::ArgMatches) -> (Bn254, u16) {
    let key_file = matches
        .get_one::<String>("key-file")
        .expect("Please provide key file");
    let port = matches
        .get_one::<String>("port")
        .expect("Please provide port");
    let key = load_key_from_file(key_file);
    let me = format!("{}@{}", key, port);
    let parts = me.split('@').collect::<Vec<&str>>();
    if parts.len() != 2 {
        panic!("Identity not well-formed");
    }
    let key = parts[0];
    let signer = get_signer_from_fr(key);

    let port = parts[1].parse::<u16>().expect("Port not well-formed");
    tracing::info!(port, "loaded port");

    (signer, port)
}

async fn get_operator_states() -> Result<Vec<QuorumInfo>, Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    
    let http_rpc = env::var("HTTP_RPC").expect("HTTP_RPC must be set");
    let ws_rpc = env::var("WS_RPC").expect("WS_RPC must be set");
    let avs_deployment_path = env::var("AVS_DEPLOYMENT_PATH").expect("AVS_DEPLOYMENT_PATH must be set");
    
    let client = EigenStakingClient::new(
        String::from(http_rpc),
        String::from(ws_rpc),
        String::from(avs_deployment_path),
    ).await?;

    client.get_operator_states().await
}

fn main() {
    // Initialize runtime
    let runtime_cfg = tokio::Config::default();
    let runner = tokio::Runner::new(runtime_cfg.clone());
    
    // Parse arguments
    let matches = Command::new("commonware-aggregation")
        .about("generate and verify BN254 Multi-Signatures")
        .arg(
            Arg::new("key-file")
                .long("key-file")
                .required(true)
                .help("Path to the YAML file containing the private key"),
        )
        .arg(
            Arg::new("port")
                .long("port")
                .required(true)
                .help("Port to run the service on"),
        )
        .arg(
            Arg::new("orchestrator")
                .long("orchestrator")
                .required(false)
                .help("Path to orchestrator key file"),
        )
        .get_matches();
    

    // Configure my identity
    let (signer, port) = configure_identity(&matches);

    // Get operator states
    
    // Start runtime
    runner.start(|context: tokio::Context| async move {
        let mut recipients = Vec::new();
        // Scoped to avoid configuring two loggers
        {
            eigen_logging::init_logger(LogLevel::Debug);
            let quorum_infos = get_operator_states().await.expect("Failed to get operator states");
            // Configure allowed peers
            let participants = quorum_infos[0].operators.clone(); //TODO: Fix hardcoded quorum_number
            if participants.len() == 0 {
                panic!("Please provide at least one participant");
            }
            for participant in participants {
                let verifier = participant.pub_keys.unwrap().g2_pub_key;
                tracing::info!(key = ?verifier, "registered authorized key",);
                recipients.push(verifier);
            }
            let test_signer = get_signer_from_fr("69");
            let test_verifier = test_signer.public_key();
            recipients.push(test_verifier);
        }
        let subscriber = tracing_subscriber::fmt()
            .with_max_level(tracing::Level::DEBUG)
            .with_writer(std::io::stdout)
            .finish();
        let _ = tracing::subscriber::set_default(subscriber);

        // Configure network
        const MAX_MESSAGE_SIZE: usize = 1024 * 1024; // 1 MB
        let my_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::UNSPECIFIED), port);
        let my_local_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), port);
        let p2p_cfg = lookup::Config::aggressive(
            signer.clone(),
            APPLICATION_NAMESPACE,
            my_addr,
            my_local_addr,
            MAX_MESSAGE_SIZE,
        );
        let (mut network, mut oracle) = Network::new(context.with_label("network"), p2p_cfg);

        // Provide authorized peers
        let mut recipients_with_addr: Vec<(bn254::PublicKey, SocketAddr)> = Vec::new();
        for participant in &quorum_infos[0].operators {
            let verifier = participant.pub_keys.as_ref().unwrap().g2_pub_key;
            if let Some(socket) = &participant.socket {
                let socket_addr = SocketAddr::from_str(socket).expect("Socket address not well-formed");
                recipients_with_addr.push((verifier, socket_addr));
            }
        }
        oracle.register(0, recipients_with_addr).await;

        // Parse contributors from operator states
        let mut contributors = Vec::new();
        let mut contributors_map = HashMap::new();
        let quorum_infos = get_operator_states().await.expect("Failed to get operator states");
        let operators = &quorum_infos[0].operators;
        if operators.len() == 0 {
            panic!("Please provide at least one contributor");
        }
        for operator in operators {
            let verifier = operator.pub_keys.as_ref().unwrap().g2_pub_key.clone();
            let verifier_g1 = operator.pub_keys.as_ref().unwrap().g1_pub_key.clone();
            tracing::info!(key = ?verifier, "registered contributor",);
            contributors.push(verifier.clone());
            contributors_map.insert(verifier, verifier_g1);
        }

        // Check if I am the orchestrator
        const DEFAULT_MESSAGE_BACKLOG: usize = 256;
        const COMPRESSION_LEVEL: Option<i32> = Some(3);

        let orchestrator_file = matches
            .get_one::<String>("orchestrator")
            .expect("No orchestrator addr");
        // Create contributor
        let (sender, receiver) = network.register(
            0,
            Quota::per_second(NZU32!(1)),
            DEFAULT_MESSAGE_BACKLOG,
        );
        let orchestrator_key = load_key_from_file(orchestrator_file);
        let orchestrator = get_signer_from_fr(&orchestrator_key).public_key();
        let contributor = handlers::Contributor::new(
            orchestrator,
            signer,
            contributors,
        );
        context.spawn(|_| async move { contributor.run(sender, receiver).await });

        let _ = network.start().await;
    });
}
