use common::{Channels, DecodedFrame, EncodedFrame, Subscription};
use crypto_common::KeyInit;
use hmac::{Hmac, Mac}; // Import Hmac and Mac from hmac
use sha2::Sha256; // Import KeyInit from crypto_common

use ascon_aead::aead::generic_array::GenericArray;
use ascon_aead::aead::Error;
use ascon_aead::aead::{AeadInPlace, KeyInit}; // Import traits correctly
use ascon_aead::{Ascon80pq, KeySizeUser};

fn get_base_frame_secret() -> [u8; 32] {
    // Base frame secret: 32 bytes of random data
    return [0x00; 32];
}

fn Derive_Payload_Secret(Frame_Payload: &[u8], timestamp: u64, channel_id: u32) -> [u8; 32] {
    //Todo: Find a way to use sha3 kmac as the hmac.
    type HmacSha256 = Hmac<Sha256>;

    let key = get_base_frame_secret();
    // Use fully-qualified syntax to specify the `new_from_slice` method from the `KeyInit` trait
    let mut mac = <HmacSha256 as KeyInit>::new_from_slice(&key).unwrap();
    mac.update(&channel_id.to_le_bytes());
    let result = mac.finalize();
    let channel_secret = result.into_bytes();

    println!("Channel Secret: {:?}", channel_secret);

    // Use fully-qualified syntax again for the second call
    let mut mac2 = <HmacSha256 as KeyInit>::new_from_slice(&channel_secret).unwrap();
    mac2.update(&timestamp.to_le_bytes());
    mac2.update(&(!timestamp).to_le_bytes());
    let result = mac2.finalize();
    let payload_secret = result.into_bytes();

    println!("Payload Secret: {:?}", payload_secret);

    payload_secret.into()
}

fn Encrypt_Frame<'a>(
    Payload_Secret: [u8; 32],
    Frame_Payload: &'a [u8],
) -> Result<EncodedFrame<'a>, Error> {
    // Convert the Payload_Secret into a GenericArray for the key
    let key = GenericArray::from_slice(&Payload_Secret);

    // Initialize the Ascon80pq cipher with the key
    let cipher = Ascon80pq::new(key);

    // Create a buffer to hold the encrypted data
    let mut buffer = Frame_Payload.to_vec();

    // Encrypt the data in place
    // The second argument is the associated data (in this case, an empty slice `""`)
    cipher.encrypt_in_place(&[0u8; 0], &mut buffer)?;

    // Wrap the encrypted data in the EncodedFrame struct
    let encoded_frame = EncodedFrame { data: &buffer };

    // Return the EncodedFrame
    Ok(encoded_frame)
}

fn Encrypt_Encrypted_Frame(
    channel_id: u32,
    timestamp: u64,
    Encrypted_Frame: EncodedFrame,
    Frame_Secret: [u8; 32],
) -> EncodedFrame {
    // AEAD_Enc(channel ID || timestamp || ~channel ID || ~timestamp || encrypted payload, "", frame secret)
}

fn main() {
    // Make default subscription sub which is impl Default
    let sub = Subscription::default();
    println!("{:?}", sub);
    // Test derive payload secret
    let frame_payload = &[0x00, 0x01, 0x02, 0x03];
    let timestamp = 0x12345678;
    let channel_id = 0x12345678;
    let payload_secret = Derive_Payload_Secret(frame_payload, timestamp, channel_id);
    println!("{:?}", payload_secret);
    //Test encrypt frame
    let encrypted_frame = Encrypt_Frame(payload_secret, frame_payload);
    println!("{:?}", encrypted_frame);
}
/* TODO: Implement these functions;
 * 32 bytes for all secret keys
 * Derive_Payload_Secret(Frame Payload, timestamp, channel_id) -> Payload_Secret
 * Encrypt_Frame(Payload_Secret, Frame Payload) -> Encrypted_Frame
 * Encrypt_Encrypted_Frame(channel_id, timestamp, Encrypted_Frame, Frame_Secret) -> Encrypted_Payload

*/
