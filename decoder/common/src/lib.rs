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

pub fn config() -> Configuration<LittleEndian, Fixint> {
    bincode::config::standard()
        .with_little_endian()
        .with_fixed_int_encoding()
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub struct BaseChannelSecret(pub [u8; LEN_BASE_CHANNEL_SECRET]);

#[derive(Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub struct BaseSubscriptionSecret(pub [u8; LEN_BASE_SUBSCRIPTION_SECRET]);

/// The Channel Secret which is given with a subscription.
#[derive(Debug, Deserialize, Serialize, Encode, Decode)]
#[serde(transparent)]
pub struct ChannelSecret(pub [u8; LEN_CHANNEL_SECRET]);

#[derive(Debug, Deserialize, Serialize, Encode, Decode)]
#[serde(transparent)]
pub struct FrameKey(pub [u8; LEN_ASCON_KEY]);

/// The Picture Key which is derived with a particular frame to encrypt the picture.
#[derive(Debug, Deserialize, Serialize, Encode, Decode)]
pub struct PictureKey(pub [u8; LEN_ASCON_KEY]);

/// The Subscription Key which is derived for a particular device and used to encrypt subscription updates.
#[derive(Debug, Deserialize, Serialize, Encode, Decode)]
#[serde(transparent)]
pub struct SubscriptionKey(pub [u8; LEN_ASCON_KEY]);

#[derive(Debug, Deserialize, Serialize)]
pub struct DeploymentSecrets {
    pub frame_key: FrameKey,
    pub base_channel_secret: BaseChannelSecret,
    pub base_subscription_secret: BaseSubscriptionSecret,
}

/// The subscription update payload received from the host.
#[derive(Debug, Decode, Encode)]
pub struct EncryptedSubscription(pub [u8; LEN_ENCRYPTED_SUBSCRIPTION]);

/// Public information about a subscription. Embedded within a StoredSubscription and primarily
/// used for serialization when communicating with the host.
#[derive(Debug, Deserialize, Serialize, Decode, Encode)]
pub struct SubscriptionInfo {
    pub channel_id: u32,
    pub start: u64,
    pub end: u64,
}

/// All information about a subscription.
#[derive(Debug, Decode, Encode)]
pub struct StoredSubscription {
    pub info: SubscriptionInfo,
    pub channel_secret: ChannelSecret,
}

/// A list of 8 optional SubscriptionInfo objects for each channel.
// TODO: This does not make sense
#[derive(Debug)]
pub struct SubscriptionInfoList {
    pub subscribed_channels: u32,
    pub subscriptions: [SubscriptionInfo; LEN_STANDARD_CHANNELS],
}

impl Encode for SubscriptionInfoList {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> core::result::Result<(), EncodeError> {
        Encode::encode(&self.subscribed_channels, encoder)?;
        for i in 0..self.subscribed_channels {
            Encode::encode(&self.subscriptions[i as usize], encoder)?;
        }
        Ok(())
    }
}

impl Decode for SubscriptionInfoList {
    fn decode<D: Decoder>(decoder: &mut D) -> core::result::Result<Self, DecodeError> {
        let mut out = SubscriptionInfoList {
            subscribed_channels: Decode::decode(decoder)?,
            subscriptions: core::array::from_fn(|_| SubscriptionInfo {
                channel_id: 0,
                start: 0,
                end: 0,
            }),
        };
        for i in 0..out.subscribed_channels {
            out.subscriptions[i as usize] = Decode::decode(decoder)?;
        }
        Ok(out)
    }
}

/// A list of 8 optional StoredSubscription objects for each channel.
// TODO: This does not make sense
#[derive(Debug, Decode, Encode)]
pub struct StoredSubscriptionList {
    pub subscribed_channels: u32,
    pub subscriptions: [StoredSubscription; LEN_STANDARD_CHANNELS],
}

// 80 bytes of frame data, 4 bytes of channel ID, 8 bytes of timestamp, 1 byte of frame length
// Plus 16 bytes from encryption
/// The frame payload received from the host.
#[derive(Debug, Decode, Encode)]
pub struct EncryptedFrame(pub [u8; LEN_ENCRYPTED_FRAME]);

/// Encrypted frame data, stored in a DecryptedFrame object.
#[derive(Debug, Decode, Encode)]
pub struct EncryptedPicture(pub [u8; LEN_ENCRYPTED_PICTURE]);

/// An object representing a frame halfway through the decryption process. It contains the
/// encrypted frame data but decrypted versions of the channel ID, timestamp, and frame length.
#[derive(Debug, Decode, Encode)]
pub struct DecryptedFrame {
    pub encrypted_picture: EncryptedPicture,
    pub channel_id: u32,
    pub timestamp: u64,
    pub frame_length: u8,
}

/// The final 64-byte decrypted frame
#[derive(Debug, Decode, Encode)]
pub struct Picture(pub [u8; LEN_PICTURE]);

#[cfg(test)]
mod tests {
    use bincode::encode_into_slice;

    use super::*;

    #[test]
    fn test_subscription_info_to_bytes() {
        let sub = SubscriptionInfo {
            channel_id: 0x1,
            start: 0x32,
            end: 0x1F4
        };
        let sub_bytes: [u8; 20] = [
            0x1, 0x0, 0x0, 0x0,
            0x32, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
            0xF4, 0x1, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0
        ];
        let mut actual_bytes = [0xff; 20];
        assert_eq!(encode_into_slice(sub, &mut actual_bytes, config()).unwrap(), 20);
        assert_eq!(actual_bytes, sub_bytes);
    }

    // #[test]
    // fn test_subscription_info_list_to_bytes() {
    //     let sub1 = SubscriptionInfo {
    //         channel_id: 0x1,
    //         start: 0x32,
    //         end: 0x1F4
    //     };

    //     let sub2 = SubscriptionInfo {
    //         channel_id: 0x02,
    //         start: 0x32,
    //         end: 0x1F4
    //     };

    //     let mut channels_bytes: [u8; 168] = [0; 168];
    //     channels_bytes[0..4].copy_from_slice(&[0x0, 0x0, 0x0, 0x2]);
    //     channels_bytes[4..24].copy_from_slice(&sub1.to_bytes());
    //     channels_bytes[24..44].copy_from_slice(&sub2.to_bytes());

    //     let subscription_info_list = SubscriptionInfoList([Some(sub1), Some(sub2), None, None, None, None, None, None]);
    //     assert_eq!(subscription_info_list.to_bytes(), channels_bytes);
    // }
}
