use serde::{Deserialize, Serialize};
use serde_json::Result;
use crate::LEN_ASCON_KEY;

#[derive(Debug, Deserialize, Serialize)]
pub struct Secrets {
    pub frame: [u8; LEN_ASCON_KEY],
    pub subscription: [u8; LEN_ASCON_KEY],
    pub channels: Vec<ChannelSecret>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ChannelSecret {
    pub id: u32,
    pub secret: [u8; LEN_ASCON_KEY],
}
