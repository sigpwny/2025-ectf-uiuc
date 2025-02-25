use ascon_aead::{Ascon128, Key, Nonce};
use ascon_aead::aead::{Aead, KeyInit};
use common::constants::{LEN_ASCON_KEY, LEN_ASCON_NONCE};
use rand::Rng;

// Returns encrypted data with a randomly generated nonce prepended to it
pub fn encrypt_ascon(data: &[u8], key_bytes: &[u8; LEN_ASCON_KEY]) -> Vec<u8> {
    let mut rng = rand::rng();
    let key = Key::<Ascon128>::from_slice(key_bytes);
    let nonce_bytes = rng.random::<[u8; LEN_ASCON_NONCE]>();
    let nonce = Nonce::<Ascon128>::from_slice(&nonce_bytes);
    let cipher = Ascon128::new(key);

    let ciphertext = cipher.encrypt(nonce, data).expect("Encryption failure!");
    let mut output = Vec::new();
    output.extend_from_slice(&nonce_bytes);
    output.extend_from_slice(&ciphertext);
    output
}