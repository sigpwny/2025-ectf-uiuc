use pyo3::prelude::*;

#[pyclass]
struct Encoder {
    secrets: Vec<u8>,
}

/// Encoder class for encoding frames.
#[pymethods]
impl Encoder {
    /// Initialize the encoder with the given secrets.
    #[new]
    fn new(secrets: Vec<u8>) -> Self {
        Encoder { secrets }
    }

    /// Encode a frame with the given channel and timestamp.
    fn encode(&self, channel: u32, frame: Vec<u8>, timestamp: u64) -> Vec<u8> {
        // TODO: Placeholder encoding logic, modify as needed
        let mut encoded = Vec::new();
        // TODO: Use secrets by doing self.secrets
        encoded.extend_from_slice(&channel.to_le_bytes());
        encoded.extend_from_slice(&timestamp.to_le_bytes());
        encoded.extend_from_slice(&frame);
        encoded
    }
}

/// Generate a subscription for a given device ID, time range, and channel.
#[pyfunction]
fn gen_subscription(secrets: Vec<u8>, device_id: u32, start: u64, end: u64, channel: u32) -> Vec<u8> {
    // TODO: Placeholder subscription generation logic, modify as needed
    let mut subscription = Vec::new();
    subscription.extend_from_slice(&device_id.to_le_bytes());
    subscription.extend_from_slice(&channel.to_le_bytes());
    subscription.extend_from_slice(&start.to_le_bytes());
    subscription.extend_from_slice(&end.to_le_bytes());
    subscription
}

/// Build the Python module.
#[pymodule]
fn ectf25_design(m: &Bound<'_, PyModule>) -> PyResult<()> {
    // Create the submodule "ectf25_design.encoder"
    let m_encoder = PyModule::new(m.py(), "ectf25_design.encoder")?;
    m_encoder.add_class::<Encoder>()?;
    m.add_submodule(&m_encoder)?;

    // Create the submodule "ectf25_design.gen_subscription"
    let m_gen_subscription = PyModule::new(m.py(), "ectf25_design.gen_subscription")?;
    m_gen_subscription.add_function(wrap_pyfunction!(gen_subscription, m)?)?;
    m.add_submodule(&m_gen_subscription)?;

    // Register the submodules in the main module to allow importing them directly
    // This is a bit of a hack: https://github.com/PyO3/pyo3/issues/759
    let modules = m.py().import("sys")?.getattr("modules")?;
    modules.set_item("ectf25_design.encoder", m_encoder)?;
    modules.set_item("ectf25_design.gen_subscription", m_gen_subscription)?;

    Ok(())
}
