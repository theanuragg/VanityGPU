use crate::attestation::Attestation;
use crate::gpu::VanityMatch;
use crate::types::EncryptionSeed;
use reqwest::Client;
use serde::Serialize;
use std::error::Error;

#[derive(Serialize)]
struct ReportPayload {
    job_id: String,
    matches: Vec<MatchResult>,
    attestation: Attestation,
}

#[derive(Serialize)]
struct MatchResult {
    pubkey: String,
    encrypted_seed: EncryptionSeed,
}

pub fn report_success(
    webhook_url: &str,
    job_id: &str,
    results: Vec<(VanityMatch, EncryptionSeed)>,
    attestation: Attestation,
) -> Result<(), Box<dyn Error>> {
    let client = Client::new();

    let matches = results
        .into_iter()
        .map(|(m, enc)| MatchResult {
            pubkey: m.pubkey.to_string(),
            encrypted_seed: enc,
        })
        .collect();

    let payload = ReportPayload {
        job_id: job_id.to_string(),
        matches,
        attestation,
    };

    // We are in a non-async main (unless we change main to async), so we need blocking client OR runtime.
    // Worker is better off being async. I will assume main becomes async.
    // BUT for now, to make it work with the current loop structure if I don't change main massive, I can use blocking.
    // `reqwest::blocking` requires `blocking` feature.
    // Let's assume we switch main to async tokio.

    // Actually, I can create a runtime just for this or use blocking.
    // Let's try to use blocking for simplicity if `reqwest` has it enabled, or just spawn a runtime.
    // "reqwest": { "features": ["json"] } in my cargo edit.
    // I should probably use `tokio::main` in main.rs.

    // For this function, I will make it async and await it in main.
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {
            client
                .post(webhook_url)
                .json(&payload)
                .send()
                .await?
                .error_for_status()?;
            Ok(())
        })
}
