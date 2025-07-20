mod bindings;
mod handlers;
mod ingress;
mod validator;
mod wire;
//use alloy_primitives::{address, hex_literal::hex};
use ark_bn254::Fr;
//use ark_ff::{Fp, PrimeField};
use bn254::Bn254;
use clap::{Arg, Command, value_parser};
use commonware_cryptography::Signer;
use commonware_p2p::authenticated::lookup::{self, Network};
use commonware_runtime::{
    Metrics, Runner, Spawner,
    tokio::{self},
};
use commonware_utils::NZU32;
//use commonware_utils::quorum;
use eigen_crypto_bls::convert_to_g1_point; //convert_to_g2_point
use governor::Quota;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::{
    net::{IpAddr, Ipv4Addr, SocketAddr},
    num::NonZeroU32,
};
use std::{str::FromStr, time::Duration};
//use tracing::instrument::WithSubscriber;
//use tracing::info;
use commonware_eigenlayer::network_configuration::{EigenStakingClient, QuorumInfo};
use std::env;
use dotenv;
use eigen_logging::log_level::LogLevel;

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
struct KeyConfig {
    privateKey: String,
}
fn get_signer_from_fr(key: &str) -> Bn254 {
    let fr = Fr::from_str(key).expect("Invalid decimal string for private key");
    Bn254::from_scalar(fr)
}

fn load_key_from_file(path: &str) -> String {
    let contents = fs::read_to_string(path).expect("Could not read key file");
    let config: KeyConfig = serde_json::from_str(&contents).expect("Could not parse key file");
    config.privateKey
}

fn get_signer(key: &str) -> Bn254 {
    let fr = Fr::from_str(key).expect("Invalid decimal string for private key");
    Bn254::from_scalar(fr)
}

// Unique namespace to avoid message replay attacks.
const APPLICATION_NAMESPACE: &[u8] = b"_COMMONWARE_AGGREGATION_";

async fn get_operator_states() -> Result<Vec<QuorumInfo>, Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    
    let http_rpc = env::var("HTTP_RPC").expect("HTTP_RPC must be set");
    let ws_rpc = env::var("WS_RPC").expect("WS_RPC must be set");
    let avs_deployment_path = env::var("AVS_DEPLOYMENT_PATH").expect("AVS_DEPLOYMENT_PATH must be set");
    println!("pre init");
    let client = EigenStakingClient::new(
        String::from(http_rpc),
        String::from(ws_rpc),
        String::from(avs_deployment_path),
    ).await?;
    println!("init passed");
    client.get_operator_states().await
}

fn main() {
    // Initialize runtime
    let runtime_cfg = tokio::Config::default();
    let runner = tokio::Runner::new(runtime_cfg.clone());

    // Parse arguments
    let matches = Command::new("orchestrator")
        .about("generate and verify BN254 Multi-Signatures")
        .arg(
            Arg::new("bootstrappers")
                .long("bootstrappers")
                .required(false)
                .value_delimiter(',')
                .value_parser(value_parser!(String)),
        )
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
        .get_matches();

    // // Create logger
    // tracing_subscriber::fmt()
    //     .with_max_level(tracing::Level::DEBUG)
    //     .init();

    // Configure my identity
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
    let signer = get_signer(key);
    tracing::info!(key = ?signer.public_key(), "loaded signer");
    let public_key = signer.public_g1();
    let (apk, _, _) = bn254::get_points(&[public_key], &[signer.public_key()], &[]).unwrap();
    let g1_point = convert_to_g1_point(apk).unwrap();
    println!(
        "public key G1 coordinates: ({}, {})",
        g1_point.X, g1_point.Y
    );
    println!("key: {}", key);
    println!("parts: {:?}", parts);
    println!("me: {:?}", me);
    // std::process::exit(0);

    // Configure my port
    let port = parts[1].parse::<u16>().expect("Port not well-formed");
    tracing::info!(port, "loaded port");

    // Configure bootstrappers (if provided)
    let bootstrappers = matches.get_many::<String>("bootstrappers");
    let mut bootstrapper_identities = Vec::new();
    if let Some(bootstrappers) = bootstrappers {
        for bootstrapper in bootstrappers {
            let parts = bootstrapper.split('@').collect::<Vec<&str>>();
            let verifier = get_signer(parts[0]).public_key();
            let bootstrapper_address =
                SocketAddr::from_str(parts[1]).expect("Bootstrapper address not well-formed");
            bootstrapper_identities.push((verifier, bootstrapper_address));
        }
    }

    // Configure network
    const MAX_MESSAGE_SIZE: usize = 1024 * 1024; // 1 MB
    let my_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::UNSPECIFIED), port);
    let p2p_cfg = lookup::Config::aggressive(
        signer.clone(),
        APPLICATION_NAMESPACE,
        my_addr,
        my_addr,
        MAX_MESSAGE_SIZE,
    );

    // Start runtime
    runner.start(|context| async move {
        let (mut network, mut oracle) = Network::new(context.with_label("network"), p2p_cfg);
        let mut recipients ;
        let quorum_infos;
        {
            eigen_logging::init_logger(LogLevel::Debug);
            // Get operator states and configure allowed peers
            quorum_infos = get_operator_states().await.expect("Failed to get operator states");
            recipients = Vec::new();
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

        // Provide authorized peers
        oracle.register(0, recipients).await;

        // Parse contributors from operator states
        let mut contributors = Vec::new();
        let mut contributors_map = HashMap::new();
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

        // Infer threshold
        let threshold = 3; //hardcoded for now

        // Run as the orchestrator
        const DEFAULT_MESSAGE_BACKLOG: usize = 256;
        const COMPRESSION_LEVEL: Option<i32> = Some(3);
        const AGGREGATION_FREQUENCY: Duration = Duration::from_secs(30);

        let (sender, receiver) = network.register(
            0,
            Quota::per_second(NZU32!(1)),
            DEFAULT_MESSAGE_BACKLOG,
        );
        let orchestrator = handlers::Orchestrator::new(
            context.clone(),
            signer,
            AGGREGATION_FREQUENCY,
            contributors,
            contributors_map,
            threshold as usize,
        ).await;

        context.spawn(|_| async move { orchestrator.run(sender, receiver).await });

        let _ = network.start().await;
    });
}
