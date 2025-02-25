
// pub struct AsconEncryptedFrame {
//     pub ascon_nonce: [u8; 16],
//     pub encrypted_data: [u8] // ????
// }

// impl Ascon {
    
// }

use ascon_aead::{Ascon128, Key, Nonce};
use ascon_aead::aead::{AeadInPlace, KeyInit};
use common::constants::{
    LEN_ASCON_KEY,
    LEN_ASCON_NONCE,
    FLASH_ADDR_FRAME_KEY,
    FLASH_ADDR_SUBSCRIPTION_KEY,
};
use common::{
    FrameKey,
    SubscriptionKey,
};

/// Get the frame key from flash memory.
pub fn get_frame_key() -> FrameKey {
    let frame_key_bytes = unsafe {
        core::ptr::read_volatile(FLASH_ADDR_FRAME_KEY as *const [u8; LEN_ASCON_KEY])
    };
    FrameKey(frame_key_bytes)
}

/// Get the subscription key from flash memory.
pub fn get_subscription_key() -> SubscriptionKey {
    let subscription_key_bytes = unsafe {
        core::ptr::read_volatile(FLASH_ADDR_SUBSCRIPTION_KEY as *const [u8; LEN_ASCON_KEY])
    };
    SubscriptionKey(subscription_key_bytes)
}

/// Decrypt the given Ascon-encrypted data using an Ascon key.
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