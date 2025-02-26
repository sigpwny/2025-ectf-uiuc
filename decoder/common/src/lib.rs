#![cfg_attr(not(test), no_std)]
// #![no_std]

pub mod constants;
pub mod crypto;

use bincode::{
    config::{Configuration, Fixint, LittleEndian},
    de::Decoder,
    enc::Encoder,
    error::{DecodeError, EncodeError},
    Decode, Encode,
};
use constants::*;
use serde::{Deserialize, Serialize};
use zeroize::{Zeroize, ZeroizeOnDrop};

pub const BINCODE_CONFIG: Configuration<LittleEndian, Fixint> = bincode::config::standard()
    .with_little_endian()
    .with_fixed_int_encoding();

/// Messages that the host sends to the decoder.
#[derive(Debug, Zeroize)]
pub enum MessageToDecoder {
    ListSubscriptions,
    UpdateSubscription(EncryptedSubscription),
    DecodeFrame(EncryptedFrame),
}

/// Messages that the decoder can send to the host.
#[derive(Debug, Zeroize, ZeroizeOnDrop)]
pub enum MessageFromDecoder {
    ListSubscriptions(SubscriptionInfoList),
    UpdateSubscription,
    DecodeFrame(SizedPicture),
    Error,
    Debug,
}

#[derive(Debug, Deserialize, Serialize, Zeroize, ZeroizeOnDrop)]
#[serde(transparent)]
pub struct BaseChannelSecret(pub [u8; LEN_BASE_CHANNEL_SECRET]);

#[derive(Debug, Deserialize, Serialize, Zeroize, ZeroizeOnDrop)]
#[serde(transparent)]
pub struct BaseSubscriptionSecret(pub [u8; LEN_BASE_SUBSCRIPTION_SECRET]);

/// The Channel Secret which is given with a subscription.
#[derive(Debug, Deserialize, Serialize, Decode, Encode, Zeroize, ZeroizeOnDrop)]
#[serde(transparent)]
pub struct ChannelSecret(pub [u8; LEN_CHANNEL_SECRET]);

#[derive(Debug, Deserialize, Serialize, Decode, Encode, Zeroize, ZeroizeOnDrop)]
#[serde(transparent)]
pub struct FrameKey(pub [u8; LEN_ASCON_KEY]);

/// The Picture Key which is derived with a particular frame to encrypt the picture.
#[derive(Debug, Deserialize, Serialize, Decode, Encode, Zeroize, ZeroizeOnDrop)]
pub struct PictureKey(pub [u8; LEN_ASCON_KEY]);

/// The Subscription Key which is derived for a particular device and used to encrypt subscription updates.
#[derive(Debug, Deserialize, Serialize, Decode, Encode, Zeroize, ZeroizeOnDrop)]
#[serde(transparent)]
pub struct SubscriptionKey(pub [u8; LEN_ASCON_KEY]);

#[derive(Debug, Deserialize, Serialize, Zeroize, ZeroizeOnDrop)]
pub struct DeploymentSecrets {
    pub frame_key: FrameKey,
    pub base_channel_secret: BaseChannelSecret,
    pub base_subscription_secret: BaseSubscriptionSecret,
}

/// The subscription update payload received from the host.
#[derive(Debug, Decode, Encode, Zeroize, ZeroizeOnDrop)]
pub struct EncryptedSubscription(pub [u8; LEN_ENCRYPTED_SUBSCRIPTION]);

/// Public information about a subscription. Embedded within a StoredSubscription and primarily
/// used for serialization when communicating with the host.
#[derive(Debug, Copy, Clone, Deserialize, Serialize, Decode, Encode, Zeroize)]
pub struct SubscriptionInfo {
    pub channel_id: u32,
    pub start: u64,
    pub end: u64,
}

/// All information about a subscription.
#[derive(Debug, Decode, Encode, Zeroize, ZeroizeOnDrop)]
pub struct StoredSubscription {
    pub info: SubscriptionInfo,
    pub channel_secret: ChannelSecret,
}

/// A list of 8 optional SubscriptionInfo objects for each channel.
#[derive(Debug, Zeroize)]
pub struct SubscriptionInfoList {
    pub num_sub_channels: u32,
    pub subscriptions: [SubscriptionInfo; LEN_STANDARD_CHANNELS],
}

impl Encode for SubscriptionInfoList {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> core::result::Result<(), EncodeError> {
        Encode::encode(&self.num_sub_channels, encoder)?;
        for i in 0..self.num_sub_channels {
            Encode::encode(&self.subscriptions[i as usize], encoder)?;
        }
        Ok(())
    }
}

impl Decode for SubscriptionInfoList {
    fn decode<D: Decoder>(decoder: &mut D) -> core::result::Result<Self, DecodeError> {
        let mut out = SubscriptionInfoList {
            num_sub_channels: Decode::decode(decoder)?,
            subscriptions: core::array::from_fn(|_| SubscriptionInfo {
                channel_id: 0,
                start: 0,
                end: 0,
            }),
        };
        for i in 0..out.num_sub_channels {
            out.subscriptions[i as usize] = Decode::decode(decoder)?;
        }
        Ok(out)
    }
}

/// A list of 8 optional StoredSubscription objects for each channel.
#[derive(Debug, Decode, Encode, Zeroize, ZeroizeOnDrop)]
pub struct StoredSubscriptionList {
    pub num_sub_channels: u32,
    pub subscriptions: [StoredSubscription; LEN_STANDARD_CHANNELS],
}

// 80 bytes of frame data, 4 bytes of channel ID, 8 bytes of timestamp, 1 byte of frame length
// Plus 16 bytes from encryption
/// The frame payload received from the host.
#[derive(Debug, Decode, Encode, Zeroize, ZeroizeOnDrop)]
pub struct EncryptedFrame(pub [u8; LEN_ENCRYPTED_FRAME]);

/// Encrypted frame data, stored in a DecryptedFrame object.
#[derive(Debug, Decode, Encode, Zeroize, ZeroizeOnDrop)]
pub struct EncryptedPicture(pub [u8; LEN_ENCRYPTED_PICTURE]);

/// An object representing a frame halfway through the decryption process. It contains the
/// encrypted frame data but decrypted versions of the channel ID, timestamp, and frame length.
#[derive(Debug, Decode, Encode, Zeroize, ZeroizeOnDrop)]
pub struct DecryptedFrame {
    pub channel_id: u32,
    pub timestamp: u64,
    pub picture_length: u8,
    pub encrypted_picture: EncryptedPicture,
}

/// The final 64-byte decrypted picture.
#[derive(Debug, Decode, Encode, Zeroize, ZeroizeOnDrop)]
pub struct Picture(pub [u8; MAX_LEN_PICTURE]);

/// The decrypted picture and its length.
#[derive(Debug, Zeroize, ZeroizeOnDrop)]
pub struct SizedPicture {
    pub picture_length: u8,
    pub picture: Picture,
}

#[derive(Debug)]
pub struct Timestamp(pub u64);

/// Returns true if the given 16 bytes are the complement of the given 16 bytes.
pub fn check_complement_16b(a: &[u8; 16], b: &[u8; 16]) -> bool {
    for i in 0..16 {
        if a[i] != !b[i] {
            return false;
        }
    }
    true
}

/// Returns the complement of the given 16 bytes.
pub fn make_complement_16b(a: &[u8; 16]) -> [u8; 16] {
    let mut b = [0u8; 16];
    for i in 0..16 {
        b[i] = !a[i];
    }
    b
}
