use super::gpu::VanityMatch;
use crate::types::EncryptionSeed;
use bs58;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use solana_sdk::signature::{Keypair, Signer};

#[derive(Serialize, Deserialize, Debug)]
pub struct Attestation {
    pub job_id: String,
    pub worker_pubkey: String,
    pub matches_hash: String,
    pub signature: String,
}

#[derive(Serialize)]
struct AttestationPayload {
    job_id: String,
    worker_pubkey: String,
    matches_digests: Vec<String>,
}

pub fn sign_attestation(
    job_id: &str,
    matches: &[(VanityMatch, EncryptionSeed)],
    worker_keypair: &Keypair,
) -> Attestation {
    let worker_pubkey = worker_keypair.pubkey().to_string();

    // Create a digest of the matches
    // We hash the (pubkey + ciphertext) for each match
    let mut matches_digests = Vec::new();
    for (m, enc) in matches {
        let mut hasher = Sha256::new();
        hasher.update(m.pubkey.to_bytes());
        hasher.update(enc.ciphertext.as_bytes());
        let hash = hasher.finalize();
        matches_digests.push(hex::encode(hash));
    }

    let payload = AttestationPayload {
        job_id: job_id.to_string(),
        worker_pubkey: worker_pubkey.clone(),
        matches_digests: matches_digests.clone(),
    };

    let message_json = serde_json::to_string(&payload).expect("serialization error");
    let signature = worker_keypair.sign_message(message_json.as_bytes());

    // Hash the whole payload to store as "attestation_hash" for quick lookup
    let mut payload_hasher = Sha256::new();
    payload_hasher.update(message_json.as_bytes());
    let matches_hash = hex::encode(payload_hasher.finalize());

    Attestation {
        job_id: job_id.to_string(),
        worker_pubkey,
        matches_hash,
        signature: bs58::encode(signature).into_string(),
    }
}
