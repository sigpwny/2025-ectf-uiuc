use bincode::decode_from_slice;
use common::{
    EncryptedFrame,
    DecryptedFrame,
    SizedPicture,
    Picture,
    Timestamp,
    BINCODE_CONFIG,
};
use common::constants::*;
use common::crypto::derive_picture_key;
use zeroize::Zeroize;
use crate::crypto::{
    decrypt_ascon,
    get_frame_key,
};
use crate::subscription::get_channel_subscription;
use hal::flc::Flc;

/// Decrypts the outer frame and returns a DecryptedFrame.
/// No metadata validation is performed.
pub fn decrypt_frame(enc_frame: &EncryptedFrame) -> Result<DecryptedFrame, ()> {
    let mut dec_frame_bytes = [0u8; LEN_DECRYPTED_FRAME];
    let mut frame_key = get_frame_key();
    match decrypt_ascon(&enc_frame.0, &frame_key.0, &mut dec_frame_bytes) {
        Ok(_) => (),
        Err(_) => return Err(()),
    }
    frame_key.zeroize();
    let dec_frame: DecryptedFrame = match decode_from_slice(&dec_frame_bytes, BINCODE_CONFIG) {
        Ok((frame, LEN_DECRYPTED_FRAME)) => frame,
        _ => return Err(()),
    };
    Ok(dec_frame)
}

/// Validates the metadata of the decrypted frame and decrypts the picture.
pub fn validate_and_decrypt_picture(
    flc: &mut Flc,
    timestamp: &mut Timestamp,
    dec_frame: &DecryptedFrame,
) -> Result<SizedPicture, ()> {
    assert!(dec_frame.picture_length as usize <= MAX_LEN_PICTURE, "Invalid picture length");
    // Get the subscription for the channel
    let mut subscription = match get_channel_subscription(flc, dec_frame.channel_id) {
        Ok(sub) => sub,
        Err(_) => return Err(()),
    };
    // Ensure the timestamp is within the subscription range
    // TODO: ENFORCE REPEATED VOLATILE CONDITIONAL
    if dec_frame.timestamp < subscription.info.start || dec_frame.timestamp > subscription.info.end {
        return Err(());
    }
    // Ensure the timestamp is greater than the last seen timestamp
    // TODO: ENFORCE REPEATED VOLATILE CONDITIONAL
    if dec_frame.timestamp <= timestamp.0 {
        return Err(());
    }
    // At this point, we have validated all the metadata
    // Update the timestamp
    timestamp.0 = dec_frame.timestamp;
    // Derive the picture key
    let mut picture_key = derive_picture_key(&subscription.channel_secret, dec_frame.timestamp);
    subscription.zeroize();
    // Decrypt the picture
    let mut dec_picture_bytes = [0u8; MAX_LEN_PICTURE];
    match decrypt_ascon(&dec_frame.encrypted_picture.0, &picture_key.0, &mut dec_picture_bytes) {
        Ok(_) => (),
        Err(_) => return Err(()),
    }
    picture_key.zeroize();
    // Initialize the plaintext picture
    let res = SizedPicture {
        picture_length: dec_frame.picture_length,
        picture: Picture(dec_picture_bytes),
    };
    Ok(res)
}