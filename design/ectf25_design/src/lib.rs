mod crypto;
use crypto::encrypt_ascon;

use common::{
    FrameKey,
    BaseChannelSecret,
    BaseSubscriptionSecret,
    DeploymentSecrets,
    SubscriptionInfo,
    StoredSubscription,
    DecryptedFrame,
    EncryptedPicture,
    BINCODE_CONFIG,
};
use common::constants::*;
use common::crypto::{
    derive_subscription_key,
    derive_channel_secret,
    derive_picture_key,
};
use pyo3::prelude::*;
use rand::Rng;

/// Generate secrets given a list of channel IDs.
#[pyfunction]
fn gen_secrets(channels: Vec<u32>) -> Vec<u8> {
    let _ = channels;
    // Generate random secrets
    let mut rng = rand::rng();
    let secrets = DeploymentSecrets {
        frame_key: FrameKey(rng.random::<[u8; LEN_ASCON_KEY]>()),
        base_channel_secret: BaseChannelSecret(rng.random::<[u8; LEN_BASE_CHANNEL_SECRET]>()),
        base_subscription_secret: BaseSubscriptionSecret(rng.random::<[u8; LEN_BASE_SUBSCRIPTION_SECRET]>()),
    };
    // Serialize the deployment secrets to JSON
    serde_json::to_vec(&secrets).expect("Failed to serialize secrets")
}

/// Generate a subscription for a given device ID, time range, and channel.
#[pyfunction]
fn gen_subscription(
    secrets: Vec<u8>,
    device_id: u32,
    start: u64,
    end: u64,
    channel: u32,
) -> Vec<u8> {
    assert!(channel <= MAX_STANDARD_CHANNEL, "Invalid channel");
    assert!(start <= end, "Invalid time range");

    // Deserialize the deployment secrets
    let s: DeploymentSecrets = serde_json::from_slice(&secrets).expect("Failed to deserialize secrets");
    // Derive the channel secret for the given channel
    let channel_secret = derive_channel_secret(&s.base_channel_secret, channel);
    // Derive the subscription encryption key for the given decoder ID
    let subscription_key = derive_subscription_key(&s.base_subscription_secret, device_id);

    // Initialize the subscription info
    let subscription_info = SubscriptionInfo {
        channel_id: channel,
        start,
        end,
    };
    let stored_subscription = StoredSubscription {
        info: subscription_info,
        channel_secret,
    };

    // Encode the subscription
    let mut subscription_bytes = [0u8; LEN_STORED_SUBSCRIPTION];
    match bincode::encode_into_slice(&stored_subscription, &mut subscription_bytes, BINCODE_CONFIG) {
        Ok(LEN_STORED_SUBSCRIPTION) => (),
        _ => panic!("Failed to encode subscription"),
    }

    // Encrypt the subscription
    let encrypted_subscription = encrypt_ascon(&subscription_bytes, &subscription_key.0);
    assert_eq!(encrypted_subscription.len(), LEN_ENCRYPTED_SUBSCRIPTION, "Invalid encrypted subscription length");
    encrypted_subscription
}

#[pyclass]
struct Encoder {
    secrets: DeploymentSecrets,
}

/// Encoder class for encoding frames.
#[pymethods]
impl Encoder {
    /// Initialize the encoder with the given secrets.
    #[new]
    fn new(secrets: Vec<u8>) -> Self {
        let s: DeploymentSecrets = serde_json::from_slice(&secrets).expect("Failed to deserialize deployment secrets");
        Encoder { secrets: s }
    }

    /// Encode a frame with the given channel and timestamp.
    fn encode(&self, channel: u32, frame: Vec<u8>, timestamp: u64) -> Vec<u8> {
        assert!(channel <= MAX_STANDARD_CHANNEL, "Invalid channel");
        assert!(frame.len() <= MAX_LEN_PICTURE, "Invalid frame length");

        // Derive the picture encryption key
        let channel_secret = derive_channel_secret(&self.secrets.base_channel_secret, channel);
        let picture_key = derive_picture_key(&channel_secret, timestamp);

        // Encrypt the picture
        let mut picture_bytes = [0u8; MAX_LEN_PICTURE];
        picture_bytes[..frame.len()].copy_from_slice(&frame);
        let encrypted_picture = encrypt_ascon(&picture_bytes, &picture_key.0);
        assert_eq!(encrypted_picture.len(), LEN_ENCRYPTED_PICTURE, "Invalid encrypted picture length");

        // Initialize the plaintext frame
        let plaintext_frame = DecryptedFrame {
            channel_id: channel,
            timestamp,
            picture_length: frame.len() as u8,
            encrypted_picture: EncryptedPicture(encrypted_picture.try_into().unwrap()),
        };
        // Encode the plaintext frame
        let mut plaintext_frame_bytes = [0u8; LEN_DECRYPTED_FRAME];
        match bincode::encode_into_slice(&plaintext_frame, &mut plaintext_frame_bytes, BINCODE_CONFIG) {
            Ok(LEN_DECRYPTED_FRAME) => (),
            _ => panic!("Failed to encode frame"),
        }

        // Encrypt the frame
        let encrypted_frame = encrypt_ascon(&plaintext_frame_bytes, &self.secrets.frame_key.0);
        assert_eq!(encrypted_frame.len(), LEN_ENCRYPTED_FRAME, "Invalid encrypted frame length");
        encrypted_frame
    }
}

/// Build the Python module.
#[pymodule]
fn rust(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(gen_secrets, m)?)?;
    m.add_function(wrap_pyfunction!(gen_subscription, m)?)?;
    m.add_class::<Encoder>()?;

    Ok(())
}
