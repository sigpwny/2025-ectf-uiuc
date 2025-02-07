#![no_std]

use serde::{de, ser};

pub type Result<T> = core::result::Result<T, Error>;

pub enum Error {
    InvalidChannel,
    InvalidFrameSize,
    InvalidTimestamp,
    InvalidSubscription,
    InvalidFrame,
    InvalidData,
    TrailingBytes,
}
