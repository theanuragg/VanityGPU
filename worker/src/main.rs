mod attestation;

mod encrypt;
mod gpu;
mod http;
mod job;
mod queue;
mod types;

use attestation::sign_attestation;
use encrypt::encrypt_seed;
use gpu::VanityEngine;
use gpu::cpu::CpuVanityEngine;
use queue::JobQueue;
use solana_sdk::signature::{Keypair, Signer};
use std::{thread, time::Duration};

fn main() {
    println!("Starting VanityGPU Worker...");

    // Generate or load worker identity
    // For now, ephemeral worker instance identity
    let worker_keypair = Keypair::new();
    println!("Worker Pubkey: {}", worker_keypair.pubkey());

    let queue = JobQueue::new();
    let engine = CpuVanityEngine::new();

    loop {
        match queue.pop_job() {
            Some(job) => {
                println!("Processing job: {}", job.job_id);

                // 1. Search (Compute)
                let matches = engine.search(
                    &job.desired_prefix,
                    job.desired_suffix.as_deref(),
                    10, // Assuming fixed count for now, job might specify it
                );

                if matches.is_empty() {
                    println!("No matches found (check timeout/prefixes)");
                    continue;
                }

                // 2. Encrypt & Zeroize (Security)
                let mut secure_results = Vec::new();
                for mut m in matches {
                    let encrypted = encrypt_seed(&mut m.seed, &job.user_encryption_pubkey);
                    secure_results.push((m, encrypted));
                }

                // 3. Attest (Proof)
                let attestation = sign_attestation(&job.job_id, &secure_results, &worker_keypair);

                // 4. Report (Delivery)
                match http::report_success(
                    &job.webhook_url,
                    &job.job_id,
                    secure_results,
                    attestation,
                ) {
                    Ok(_) => println!("Job {} reported successfully", job.job_id),
                    Err(e) => println!("Failed to report job {}: {}", job.job_id, e),
                }
            }
            None => {
                // No job, wait a bit (though blpop handles wait, extra safety)
                thread::sleep(Duration::from_millis(100));
            }
        }
    }
}
