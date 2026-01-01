use solana_sdk::signature::Keypair;

pub fn find_vanity(prefix: &str) -> Keypair {
    loop {
        let kp = Keypair::new();
        let pubkey = kp.pubkey().to_string();
        if pubkey.starts_with(prefix) {
            return kp;
        }
    }
}