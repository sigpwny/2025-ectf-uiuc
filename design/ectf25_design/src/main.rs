use ascon_aead::aead::generic_array::GenericArray;
use ascon_aead::aead::Error;
use ascon_aead::aead::{AeadInPlace, KeyInit};
use ascon_aead::Ascon128; // Using Ascon128
use hmac::{Hmac, Mac};
use rand::rngs::OsRng;
use rand::TryRngCore;
use sha3::Sha3_256; // Using SHA-3 instead of SHA-256

type HmacSha3 = Hmac<Sha3_256>; // HMAC with SHA-3

/// Public information about a subscription. Embedded within a StoredSubscription and primarily
/// used for serialization when communicating with the host.
#[derive(Debug)]
pub struct SubscriptionInfo {
    channel_id: u32,
    start: u64,
    end: u64,
}

/// All information about a subscription.
#[derive(Debug)]
pub struct StoredSubscription {
    info: SubscriptionInfo,
    channel_secret: [u8; 32],
}

/// The frame payload received from the host.
#[derive(Debug)]
pub struct EncryptedFrame([u8; 109]);

/// Encrypted frame data, stored in a DecryptedFrame object.
#[derive(Debug)]
pub struct EncryptedPicture([u8; 80]);

fn get_base_picture_secret() -> [u8; 32] {
    [0x00; 32]
}

fn derive_picture_secret(picture_data: &[u8], timestamp: u64, channel_id: u32) -> [u8; 32] {
    let key = get_base_picture_secret();
    let mut mac = <HmacSha3 as KeyInit>::new_from_slice(&key).unwrap();
    mac.update(&channel_id.to_le_bytes());
    let channel_secret = mac.finalize().into_bytes();

    let mut mac2 = <HmacSha3 as KeyInit>::new_from_slice(&channel_secret).unwrap();
    mac2.update(&timestamp.to_le_bytes());
    mac2.update(&(!timestamp).to_le_bytes());
    let picture_secret = mac2.finalize().into_bytes();

    picture_secret.into()
}
fn encrypt_picture(
    picture_secret: [u8; 32],
    picture_data: [u8; 80],
) -> Result<EncryptedPicture, Error> {
    // AEAD_Enc(picture, "", picture secret)
    let mut nonce = [0u8; 16];
    OsRng.try_fill_bytes(&mut nonce).unwrap(); // Fill the nonce with random bytes

    // Truncate the 32-byte picture_secret to 16 bytes for Ascon128
    let mut key = [0u8; 16];
    key.copy_from_slice(&picture_secret[..16]);

    // Create the cipher instance with the 16-byte key
    let cipher = Ascon128::new(GenericArray::from_slice(&key));

    // Convert the picture_data array to a mutable Vec<u8> for in-place encryption
    let mut buffer = picture_data.to_vec();

    // Resize the buffer to accommodate the authentication tag (16 bytes)
    buffer.resize(buffer.len() + 16, 0);

    // Encrypt the buffer in place
    cipher.encrypt_in_place(
        GenericArray::from_slice(&nonce), // Nonce
        &[],                              // Associated data (empty in this case)
        &mut buffer,                      // Buffer to encrypt
    )?;

    // Extract the ciphertext (excluding the authentication tag)
    let encrypted_data: [u8; 80] = buffer[..80].try_into().map_err(|_| Error)?;

    // Return the encrypted picture
    Ok(EncryptedPicture(encrypted_data))
}
fn encrypt_encrypted_frame(
    channel_id: u32,
    timestamp: u64,
    encrypted_picture: EncryptedPicture,
    picture_secret: [u8; 32],
) -> Result<EncryptedFrame, Error> {
    // AEAD_Enc(channel ID || timestamp || ~channel ID || ~timestamp || encrypted payload, "", frame secret)
    let mut nonce = [0u8; 16];
    OsRng.try_fill_bytes(&mut nonce).unwrap(); // Fill the nonce with random bytes

    // Truncate the 32-byte picture_secret to 16 bytes for Ascon128
    let mut key = [0u8; 16];
    key.copy_from_slice(&picture_secret[..16]);

    // Create the cipher instance with the 16-byte key
    let cipher = Ascon128::new(GenericArray::from_slice(&key));

    // Prepare the plaintext: channel ID || timestamp || ~channel ID || ~timestamp || encrypted payload
    let mut plaintext = Vec::new();
    plaintext.extend_from_slice(&channel_id.to_le_bytes()); // 4 bytes
    plaintext.extend_from_slice(&timestamp.to_le_bytes()); // 8 bytes
    plaintext.extend_from_slice(&(channel_id ^ 0xFFFFFFFF).to_le_bytes()); // 4 bytes
    plaintext.extend_from_slice(&(timestamp ^ 0xFFFFFFFFFFFFFFFF).to_le_bytes()); // 8 bytes
    plaintext.extend_from_slice(&encrypted_picture.0); // 80 bytes

    // Ensure buffer is the correct size before encryption
    let expected_size = 109; // Total size including tag
    let mut buffer = plaintext.clone();
    buffer.resize(expected_size - 16, 0); // Resize to 93 bytes to accommodate the 16-byte tag
    buffer.resize(expected_size, 0); // Ensure it's exactly 109 bytes

    // Encrypt the buffer in place
    cipher.encrypt_in_place(
        GenericArray::from_slice(&nonce), // Nonce
        &[],                              // Associated data (empty in this case)
        &mut buffer,                      // Buffer to encrypt
    )?;

    // Convert Vec<u8> to [u8; 109] safely
    let encrypted_data: [u8; 109] = buffer.try_into().map_err(|_| Error)?;

    // Return the encrypted frame
    Ok(EncryptedFrame(encrypted_data))
}

fn main() {
    let sub_info = SubscriptionInfo {
        channel_id: 0x12345678,
        start: 0x12345678,
        end: 0x87654321,
    };
    let stored_sub = StoredSubscription {
        info: sub_info,
        channel_secret: get_base_picture_secret(),
    };

    println!("Stored Subscription: {:?}", stored_sub);

    let picture_data = [0x00; 80];
    let timestamp = 0x12345678;
    let channel_id = stored_sub.info.channel_id;
    let picture_secret = derive_picture_secret(&picture_data, timestamp, channel_id);

    match encrypt_picture(picture_secret, picture_data) {
        Ok(encrypted_picture) => {
            println!("Encrypted Picture: {:?}", encrypted_picture);
            match encrypt_encrypted_frame(
                channel_id,
                timestamp,
                encrypted_picture,
                stored_sub.channel_secret,
            ) {
                Ok(encrypted_frame) => {
                    println!("Encrypted Frame: {:?}", encrypted_frame);
                }
                Err(e) => {
                    eprintln!("Failed to encrypt frame: {:?}", e);
                }
            }
        }
        Err(e) => {
            eprintln!("Failed to encrypt picture: {:?}", e);
        }
    }
}
