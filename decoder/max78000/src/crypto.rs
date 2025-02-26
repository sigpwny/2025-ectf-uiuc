use ascon_aead::{Ascon128, Key, Nonce};
use ascon_aead::aead::{AeadInPlace, KeyInit};
use ascon_aead::aead::heapless::Vec;
use common::constants::{
    FLASH_ADDR_FRAME_KEY,
    FLASH_ADDR_SUBSCRIPTION_KEY,
    LEN_ASCON_AEAD_OVERHEAD,
    LEN_ASCON_KEY,
    LEN_ASCON_NONCE,
};
use common::{
    FrameKey,
    SubscriptionKey,
};
use zeroize::Zeroize;

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
pub fn decrypt_ascon(ascon_data: &[u8], key_bytes: &[u8; LEN_ASCON_KEY], output_bytes: &mut [u8]) -> Result<(), ()> {
    assert!(ascon_data.len() >= LEN_ASCON_AEAD_OVERHEAD);
    let nonce_bytes = &ascon_data[..LEN_ASCON_NONCE];
    let data_bytes = &ascon_data[LEN_ASCON_NONCE..];

    let key = Key::<Ascon128>::from_slice(key_bytes);
    let nonce = Nonce::<Ascon128>::from_slice(nonce_bytes);
    let cipher = Ascon128::new(key);
    let mut output_vec: Vec<u8, 256> = Vec::new();
    // Add the encrypted data to the output for decryption in place
    for byte in data_bytes {
        output_vec.push(*byte).unwrap();
    }

    // Decrypt the data in place
    match cipher.decrypt_in_place(nonce, b"", &mut output_vec) {
        Ok(_) => (),
        Err(_) => return Err(()),
    }

    // If the lengths of the vec and the output_bytes array are not the same, return an error
    if output_vec.len() != output_bytes.len() {
        return Err(());
    }

    // Copy the decrypted heapless vec into the output_bytes array
    for i in 0..output_vec.len() {
        output_bytes[i] = output_vec[i];
    }

    output_vec.zeroize();

    Ok(())
}