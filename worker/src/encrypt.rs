use crate::types::EncryptionSeed;
use aes_gcm::{
    Aes256Gcm, KeyInit,
    aead::{Aead, OsRng},
};
use base64::{Engine as _, engine::general_purpose};
use hkdf::Hkdf;
use rand::RngCore;
use sha2::Sha256;
use x25519_dalek::{EphemeralSecret, PublicKey};
use zeroize::Zeroize;

pub fn encrypt_seed(seed: &mut [u8; 32], user_pubkey_b64: &str) -> EncryptionSeed {
    // decode user pubkey
    let user_pubkey_bytes = general_purpose::STANDARD
        .decode(user_pubkey_b64)
        .expect("invalid user pubkey");
    let user_pubkey = PublicKey::from(
        <[u8; 32]>::try_from(user_pubkey_bytes.as_slice()).expect("invalid pubkey len"),
    );

    // ephemeral keypair
    let eph_secret = EphemeralSecret::random_from_rng(OsRng);
    let eph_public = PublicKey::from(&eph_secret);

    // derive shared secret
    let shared_secret = eph_secret.diffie_hellman(&user_pubkey);

    // derive AES key
    let hk = Hkdf::<Sha256>::new(None, shared_secret.as_bytes());
    let mut aes_key = [0u8; 32];
    hk.expand(b"vanitygpu", &mut aes_key).expect("hkdf error");

    // encrypt seed
    let cipher = Aes256Gcm::new_from_slice(&aes_key).unwrap();
    let mut nonce_bytes = [0u8; 12];
    OsRng.fill_bytes(&mut nonce_bytes);
    let nonce = aes_gcm::Nonce::from_slice(&nonce_bytes);

    let ciphertext = cipher
        .encrypt(nonce, seed.as_slice())
        .expect("encryption failure");

    // zeroize sensitive data
    seed.zeroize();
    aes_key.zeroize();
    // note: shared_secret and eph_secret should be zeroized on drop automatically by dalek

    // return encryption seed
    EncryptionSeed {
        ephemeral_pubkey: general_purpose::STANDARD.encode(eph_public.as_bytes()),
        nonce: general_purpose::STANDARD.encode(nonce_bytes),
        ciphertext: general_purpose::STANDARD.encode(ciphertext),
    }
}
