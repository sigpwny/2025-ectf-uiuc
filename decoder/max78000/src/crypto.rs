
// pub struct AsconEncryptedFrame {
//     pub ascon_nonce: [u8; 16],
//     pub encrypted_data: [u8] // ????
// }

// impl Ascon {
    
// }

use ascon_aead::{Ascon128, Key, Nonce};
use ascon_aead::aead::{AeadInPlace, KeyInit};
use common::constants::{LEN_ASCON_KEY, LEN_ASCON_NONCE};

pub fn decrypt_ascon(ascon_data: &[u8], key_bytes: &[u8; LEN_ASCON_KEY], output: &mut [u8]) -> Result<(), ()> {
    // if ascon_data.len() < LEN_ASCON_NONCE + AEAD_OVERHEAD {
    //     return Err(());
    // }
    let nonce_bytes = &ascon_data[..LEN_ASCON_NONCE];
    let encrypted_data = &ascon_data[LEN_ASCON_NONCE..];
    // TODO: Decrypt the encrypted data using the key and nonce
    let key = Key::<Ascon128>::from_slice(key_bytes);
    let nonce = Nonce::<Ascon128>::from_slice(nonce_bytes);
    let cipher = Ascon128::new(key);
    // let mut 
    Ok(())
}