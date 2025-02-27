use ascon_sys::crypto_aead_decrypt;
use common::constants::{
    FLASH_ADDR_FRAME_KEY, FLASH_ADDR_SUBSCRIPTION_KEY, LEN_ASCON_AEAD_OVERHEAD, LEN_ASCON_KEY,
    LEN_ASCON_NONCE, LEN_ASCON_TAG,
};
use common::{FrameKey, SubscriptionKey};

/// The error types that can be encountered during decryption
pub enum DecryptError {
    InvalidCiphertext,
}

/// Get the frame key from flash memory.
pub fn get_frame_key() -> FrameKey {
    let frame_key_bytes =
        unsafe { core::ptr::read_volatile(FLASH_ADDR_FRAME_KEY as *const [u8; LEN_ASCON_KEY]) };
    FrameKey(frame_key_bytes)
}

/// Get the subscription key from flash memory.
pub fn get_subscription_key() -> SubscriptionKey {
    let subscription_key_bytes = unsafe {
        core::ptr::read_volatile(FLASH_ADDR_SUBSCRIPTION_KEY as *const [u8; LEN_ASCON_KEY])
    };
    SubscriptionKey(subscription_key_bytes)
}

pub fn internal_decrypt_ascon(
    ciphertext: &[u8],
    nonce: &[u8; LEN_ASCON_NONCE],
    key: &[u8; LEN_ASCON_KEY],
    message: &mut [u8],
) -> Result<usize, DecryptError> {
    assert!(message.len() + LEN_ASCON_TAG >= ciphertext.len());

    let mut mlen: u64 = 0;
    let result = unsafe {
        crypto_aead_decrypt(
            message.as_mut_ptr(),
            &mut mlen,
            core::ptr::null_mut(),
            ciphertext.as_ptr(),
            ciphertext.len() as u64,
            core::ptr::null_mut(),
            0,
            nonce.as_ptr(),
            key.as_ptr(),
        )
    };

    match result {
        -1 => Err(DecryptError::InvalidCiphertext),
        0 => Ok(mlen as usize),
        _ => unreachable!("Ascon returned unexpected length"),
    }
}

/// Decrypt the given Ascon-encrypted data using an Ascon key.
pub fn decrypt_ascon(
    ascon_data: &[u8],
    key: &[u8; LEN_ASCON_KEY],
    output_bytes: &mut [u8],
) -> Result<usize, DecryptError> {
    assert!(ascon_data.len() >= LEN_ASCON_AEAD_OVERHEAD);
    let nonce = &ascon_data[..LEN_ASCON_NONCE];
    let ciphertext = &ascon_data[LEN_ASCON_NONCE..];

    internal_decrypt_ascon(ciphertext, nonce.try_into().unwrap(), key, output_bytes)
}
