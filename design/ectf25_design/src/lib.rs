use common::{
    FrameKey,
    BaseChannelSecret,
    BaseSubscriptionSecret,
    DeploymentSecrets,
};
use common::constants::{
    LEN_ASCON_KEY,
    LEN_BASE_CHANNEL_SECRET,
    LEN_BASE_SUBSCRIPTION_SECRET,
    MAX_STANDARD_CHANNEL,
};
use pyo3::prelude::*;
use rand::Rng;

/// Generate secrets given a list of channel IDs.
#[pyfunction]
fn gen_secrets(channels: Vec<u32>) -> Vec<u8> {
    let _ = channels;
    let mut rng = rand::rng();
    let secrets = DeploymentSecrets {
        frame_key: FrameKey(rng.random::<[u8; LEN_ASCON_KEY]>()),
        base_channel_secret: BaseChannelSecret(rng.random::<[u8; LEN_BASE_CHANNEL_SECRET]>()),
        base_subscription_secret: BaseSubscriptionSecret(rng.random::<[u8; LEN_BASE_SUBSCRIPTION_SECRET]>()),
    };

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
    // TODO: Placeholder subscription generation logic, modify as needed
    let mut subscription = Vec::new();
    subscription.extend_from_slice(&device_id.to_le_bytes());
    subscription.extend_from_slice(&channel.to_le_bytes());
    subscription.extend_from_slice(&start.to_le_bytes());
    subscription.extend_from_slice(&end.to_le_bytes());
    subscription
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
        let deserialized = serde_json::from_slice(&secrets).expect("Failed to deserialize secrets");
        Encoder { secrets: deserialized }
    }

    /// Encode a frame with the given channel and timestamp.
    fn encode(&self, channel: u32, frame: Vec<u8>, timestamp: u64) -> Vec<u8> {
        // TODO: Placeholder encoding logic, modify as needed
        let mut encoded = Vec::new();
        if channel > MAX_STANDARD_CHANNEL {
            panic!("Invalid channel ID");
        }
        // TODO: Use secrets by doing self.secrets
        encoded.extend_from_slice(&channel.to_le_bytes());
        encoded.extend_from_slice(&timestamp.to_le_bytes());
        encoded.extend_from_slice(&frame);
        encoded
    }
}

/// Build the Python module.
#[pymodule]
fn ectf25_design(m: &Bound<'_, PyModule>) -> PyResult<()> {
    // Create the submodule "ectf25_design.gen_secrets"
    let m_gen_secrets = PyModule::new(m.py(), "ectf25_design.gen_secrets")?;
    m_gen_secrets.add_function(wrap_pyfunction!(gen_secrets, m)?)?;
    m.add_submodule(&m_gen_secrets)?;

    // Create the submodule "ectf25_design.gen_subscription"
    let m_gen_subscription = PyModule::new(m.py(), "ectf25_design.gen_subscription")?;
    m_gen_subscription.add_function(wrap_pyfunction!(gen_subscription, m)?)?;
    m.add_submodule(&m_gen_subscription)?;

    // Create the submodule "ectf25_design.encoder"
    let m_encoder = PyModule::new(m.py(), "ectf25_design.encoder")?;
    m_encoder.add_class::<Encoder>()?;
    m.add_submodule(&m_encoder)?;

    // Register the submodules in the main module to allow importing them directly
    // This is a bit of a hack: https://github.com/PyO3/pyo3/issues/759
    let submodules = m.py().import("sys")?.getattr("modules")?;
    submodules.set_item("ectf25_design.gen_secrets", m_gen_secrets)?;
    submodules.set_item("ectf25_design.gen_subscription", m_gen_subscription)?;
    submodules.set_item("ectf25_design.encoder", m_encoder)?;

    Ok(())
}
