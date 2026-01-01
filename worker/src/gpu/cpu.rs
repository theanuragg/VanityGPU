use rand::rngs::OsRng;
use rand::RngCore;
use solana_sdk::{pubkey::Pubkey, signature::Keypair};
use bs58;

use super::{VanityEngine, VanityMatch};

pub struct CpuVanityEngine;

impl CpuVanityEngine {
    pub fn new() -> Self {
        Self
    }
}

impl VanityEngine for CpuVanityEngine {
    fn search(
        &self,
        prefix: &str,
        suffix: Option<&str>,
        max_results: usize,
    ) -> Vec<VanityMatch> {
        let mut results = Vec::new();

        while results.len() < max_results {
            let mut seed = [0u8; 32];
            OsRng.fill_bytes(&mut seed);

            let keypair = Keypair::from_seed(&seed).unwrap();
            let pubkey = keypair.pubkey();
            let base58 = pubkey.to_string();

            if !base58.starts_with(prefix) {
                continue;
            }

            if let Some(s) = suffix {
                if !base58.ends_with(s) {
                    continue;
                }
            }

            results.push(VanityMatch {
                pubkey,
                seed,
            });
        }

        results
    }
}
