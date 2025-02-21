#![cfg_attr(not(any(feature = "std", test)), no_std)]

/// secrets module is used by both the ectf25_design package and build scripts
#[cfg(feature = "std")]
pub mod secrets;

pub const LEN_ASCON_KEY: usize = 16;
pub const MAX_STANDARD_CHANNEL: u32 = 8;

// Channel Secret Wrapper
#[derive(Debug)]
pub struct ChannelSecret([u8; 32]);

/// The subscription update payload received from the host.
#[derive(Debug)]
pub struct EncryptedSubscription([u8; 68]);

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
    channel_secret: ChannelSecret,
}

/// A list of 8 optional SubscriptionInfo objects for each channel.
#[derive(Debug)]
pub struct SubscriptionInfoList([Option<SubscriptionInfo>; 8]);

/// A list of 8 optional StoredSubscription objects for each channel.
#[derive(Debug)]
pub struct StoredSubscriptionList([Option<StoredSubscription>; 8]);

// 80 bytes of frame data, 4 bytes of channel ID, 8 bytes of timestamp, 1 byte of frame length
// Plus 16 bytes from encryption
/// The frame payload received from the host.
#[derive(Debug)]
pub struct EncryptedFrame([u8; 109]);

/// Encrypted frame data, stored in a DecryptedFrame object.
#[derive(Debug)]
pub struct EncryptedPicture([u8; 80]);

/// An object representing a frame halfway through the decryption process. It contains the
/// encrypted frame data but decrypted versions of the channel ID, timestamp, and frame length.
#[derive(Debug)]
pub struct DecryptedFrame {
    encrypted_picture: EncryptedPicture,
    channel_id: u32,
    timestamp: u64,
    frame_length: u8,
}

/// The final 64-byte decrypted frame
#[derive(Debug)]
pub struct Picture([u8; 64]);

/// A trait that allows an object to be serialized to and deserialized from a fixed-size byte
/// array.
pub trait BytesSerializable<const L: usize> {
    fn to_bytes(&self) -> [u8; L];
    fn from_bytes(bytes: [u8; L]) -> Self;
}

/// Follows the format:
/// \[32-bit channel ID]\[64-bit start timestamp]\[64-bit end timestamp]
/// or:
/// \[4 bytes]\[8 bytes]\[8 bytes]
impl BytesSerializable<20> for SubscriptionInfo {
    fn to_bytes(&self) -> [u8; 20] {
        let mut bytes = [0; 20];

        bytes[..4].copy_from_slice(&self.channel_id.to_be_bytes());
        bytes[4..12].copy_from_slice(&self.start.to_be_bytes());
        bytes[12..].copy_from_slice(&self.end.to_be_bytes());

        bytes
    }

    fn from_bytes(bytes: [u8; 20]) -> SubscriptionInfo {
        let channel_id = u32::from_be_bytes(bytes[..4].try_into().unwrap());
        let start = u64::from_be_bytes(bytes[4..12].try_into().unwrap());
        let end = u64::from_be_bytes(bytes[12..].try_into().unwrap());

        SubscriptionInfo {
            channel_id,
            start,
            end
        }
    }
}

impl BytesSerializable<52> for StoredSubscription {
    fn to_bytes(&self) -> [u8; 52] {
        let mut bytes = [0; 52];

        bytes[..20].copy_from_slice(&self.info.to_bytes());
        bytes[20..].copy_from_slice(&self.channel_secret);

        bytes
    }

    fn from_bytes(bytes: [u8; 52]) -> Self {
        let info = SubscriptionInfo::from_bytes(bytes[..20].try_into().unwrap());
        let mut channel_secret = [0; 32];
        channel_secret.copy_from_slice(&bytes[20..]);

        StoredSubscription {
            info,
            channel_secret
        }
    }
}

impl BytesSerializable<168> for SubscriptionInfoList {
    fn to_bytes(&self) -> [u8; 168] {
        let mut bytes = [0; 168];

        // The first 8 bytes is the number of valid subscriptions
        let mut num_valid: u32 = 0;
        for i in 0..8 {
            if self.0[i].is_some() {
                num_valid += 1;
            }
        }

        bytes[..4].copy_from_slice(&num_valid.to_be_bytes());

        // The rest of the bytes are the serialized subscriptions
        let mut offset = 4;
        for i in 0..8 {
            if let Some(sub) = self.0[i].as_ref() {
                bytes[offset..offset + 20].copy_from_slice(&sub.to_bytes());
            }
            offset += 20;
        }

        bytes
    }

    fn from_bytes(bytes: [u8; 168]) -> Self {
        let mut subscription_info_list = [const{ None }; 8];

        let num_valid = usize::from_be_bytes(bytes[..4].try_into().unwrap());
        let mut offset = 4;
        for i in 0..num_valid {
            subscription_info_list[i] = Some(SubscriptionInfo::from_bytes(bytes[offset..offset + 20].try_into().unwrap()));
            offset += 20;
        }

        SubscriptionInfoList(subscription_info_list)
    }
}

impl BytesSerializable<417> for StoredSubscriptionList {
    fn to_bytes(&self) -> [u8; 417] {
        let mut bytes = [0; 417];

        // The first byte is a bitmask that indicates which subscriptions are present
        let mut bitmask = 0;
        for i in 0..8 {
            if self.0[i].is_some() {
                bitmask |= 1 << i;
            }
        }
        bytes[0] = bitmask;

        // The rest of the bytes are the serialized subscriptions
        let mut offset = 1;
        for i in 0..8 {
            if let Some(sub) = self.0[i].as_ref() {
                bytes[offset..offset + 52].copy_from_slice(&sub.to_bytes());
            }
            offset += 52;
        }

        bytes
    }

    fn from_bytes(bytes: [u8; 417]) -> Self {
        let mut stored_subscription_list = [const{ None }; 8];

        let bitmask = bytes[0];
        let mut offset = 1;
        for i in 0..8 {
            if bitmask & (1 << i) != 0 {
                stored_subscription_list[i] = Some(StoredSubscription::from_bytes(bytes[offset..offset + 52].try_into().unwrap()));
            }
            offset += 52;
        }

        StoredSubscriptionList(stored_subscription_list)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_subscription_info_to_bytes() {
        let sub = SubscriptionInfo {
            channel_id: 0x1,
            start: 0x32,
            end: 0x1F4
        };
        let sub_bytes: [u8; 20] = [
            0x0, 0x0, 0x0, 0x1,
            0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x32,
            0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x1, 0xF4
        ];
        assert_eq!(sub.to_bytes(), sub_bytes);
    }

    #[test]
    fn test_subscription_info_list_to_bytes() {
        let sub1 = SubscriptionInfo {
            channel_id: 0x1,
            start: 0x32,
            end: 0x1F4
        };

        let sub2 = SubscriptionInfo {
            channel_id: 0x02,
            start: 0x32,
            end: 0x1F4
        };

        let mut channels_bytes: [u8; 168] = [0; 168];
        channels_bytes[0..4].copy_from_slice(&[0x0, 0x0, 0x0, 0x2]);
        channels_bytes[4..24].copy_from_slice(&sub1.to_bytes());
        channels_bytes[24..44].copy_from_slice(&sub2.to_bytes());

        let subscription_info_list = SubscriptionInfoList([Some(sub1), Some(sub2), None, None, None, None, None, None]);
        assert_eq!(subscription_info_list.to_bytes(), channels_bytes);
    }
}
