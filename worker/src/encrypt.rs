use super::EncryptionSeed;
use aes_gcm::{Aes256Gcm, KeyInit, OsRng, aead::Aead};
use base64::{Engine as _, engine::general_purpose};
use hkdf::Hkdf;
use sha2::Sha256;
use x25519_dalek::{EphemeralSecret, PublicKey, StaticSecret};
use zeroize::Zeroize;

pub fn encrypt_seed(seed: &mut [u8; 32], user_pubkey_b64: &str) -> EncryptionSeed {
    // decode  user pubkey
    let user_pubkey_bytes = general_purpose::STANDARD
        .decode(user_pubkey_bytes)
        .expect("invaild user pubkey");
    let user_pubkey = PublicKey::from(<[u8; 32]>::try_from(user_pubkey_bytes).unwrap());

    // ephermeral keypairs
    let eph_secret = EphemeralSecret::random_from_rng(OsRng);
    let eph_public = PublicKey::from(&eph_secret);

    // derive shared secret
    let shared_secret = eph_secret.diffie_hellman(&user_pubkey);

    // derive AES key
    let hk = Hkdf::<Sha256>::new(None, &shared_secret);
    let mut key = [0u8; 32];
    hk.expand(b"vanitygpu", &mut key).expect("hkdf error");

    // encrypt seed
    let cipher = Aes256Gcm::new_from_slice(&aes_key).unwrap();
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng);
    let ciphertext = cipher.encrypt(&nonce, seed.as_slice().unwrap());

    // zeroize sensitive data
    seed.zeroize();
    aes_key.zeroize();

    // return encryption seed
    EncryptionSeed {
        ephemeral_pubkey: general_purpose::STANDARD.encode(eph_pubkey.as_bytes()),
        nonce: general_purpose::STANDARD.encode(nonce),
        ciphertext: general_purpose::STANDARD.encode(ciphertext),
    }
}
