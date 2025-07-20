use commonware_avs_router::validator::Validator;
use bn254::{
    self, Bn254, PublicKey, Signature as Bn254Signature,
};
use commonware_cryptography::Signer;
use commonware_p2p::{Receiver, Sender};
use commonware_utils::hex;
use dotenv::dotenv;
use bytes::Bytes;
use commonware_codec::{EncodeSize, ReadExt, Write};
use std::collections::{HashMap, HashSet};
use tracing::info;
use anyhow::Result;

use commonware_avs_router::wire::{self, aggregation::Payload};

pub struct Contributor {
    orchestrator: PublicKey,
    signer: Bn254,
    me: usize,
}

impl Contributor {
    pub fn new(
        orchestrator: PublicKey,
        signer: Bn254,
        mut contributors: Vec<PublicKey>,
    ) -> Self {
        dotenv().ok();
        contributors.sort();
        let mut ordered_contributors = HashMap::new();
        for (idx, contributor) in contributors.iter().enumerate() {
            ordered_contributors.insert(contributor.clone(), idx);
        }
        let me = *ordered_contributors.get(&signer.public_key()).unwrap();
        Self {
            orchestrator,
            signer,
            me,
        }
    }

    pub async fn run(
        mut self,
        mut sender: impl Sender,
        mut receiver: impl Receiver<PublicKey = PublicKey>,
    ) -> Result<()> {
        let mut signed = HashSet::new();
        let mut signatures: HashMap<u64, HashMap<usize, Bn254Signature>> = HashMap::new();
        let validator = Validator::new().await?;

        while let Ok((s, message)) = receiver.recv().await {
            // Parse message
            let Ok(message) = wire::Aggregation::read(&mut std::io::Cursor::new(message)) else {
                continue;
            };
            let round = message.round;

            // Handle message from orchestrator
            match message.payload {
                Some(Payload::Start) => (),
                _ => continue,
            };
            if s != self.orchestrator {
                info!("not from orchestrator: {:?}", s);
                continue;
            }

            // Check if already signed at round
            if !signed.insert(round) {
                info!("already signed at round: {:?}", round);
                continue;
            }
            let mut buf = Vec::with_capacity(message.encode_size());
            message.write(&mut buf);
            let payload = validator.validate_and_return_expected_hash(&buf).await?;
            info!(
                "Generating signature for round: {}, payload hash: {}",
                round,
                hex(&payload)
            );
            let signature = self.signer.sign(None, &payload);

            // Store signature
            signatures
                .entry(round)
                .or_default()
                .insert(self.me, signature.clone());

            // Return signature to orchestrator
            let message = wire::Aggregation {
                round: round,
                var1: message.var1.clone(),
                var2: message.var2.clone(),
                var3: message.var3.clone(),
                payload: Some(Payload::Signature(signature.to_vec())),
            };
            let mut buf = Vec::with_capacity(message.encode_size());
            message.write(&mut buf);
            info!("Sending signature for round: {}", round);

            // Broadcast to all (including orchestrator)
            sender
                .send(commonware_p2p::Recipients::All, Bytes::from(buf), true)
                .await
                .map_err(|e| anyhow::anyhow!("Failed to broadcast signature: {}", e))?;
            info!(round, "broadcast signature");
        }
        
        Ok(())
    }
}
