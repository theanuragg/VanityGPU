use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EncryptionSeed {
    pub ephemeral_pubkey: String,
    pub nonce: String,
    pub ciphertext: String,
}
