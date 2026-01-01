pub fn encrypt_private_key(
    private_key: &[u8],
    user_pubkey: &[u8],
) -> Vec<u8> {
    // x25519 + AES-GCM
}