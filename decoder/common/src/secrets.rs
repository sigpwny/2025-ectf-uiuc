use serde::{Deserialize, Serialize};
use serde_json::Result;

pub const LEN_SECRET_BYTES: usize = 32;

#[derive(Debug, Deserialize, Serialize)]
pub struct Secrets {
    pub frame: [u8; LEN_SECRET_BYTES],
    pub subscription: [u8; LEN_SECRET_BYTES],
    pub channels: Vec<ChannelSecret>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ChannelSecret {
    pub id: u32,
    pub secret: [u8; LEN_SECRET_BYTES],
}
