pub mod cpu;

use solana_sdk::pubkey::Pubkey;

pub struct VanityMatch {
    pub pubkey: Pubkey,
    pub seed: [u8; 32],
}

pub trait VanityEngine {
    fn search(&self, prefix: &str, suffix: Option<&str>, max_results: usize) -> Vec<VanityMatch>;
}
