#![cfg_attr(not(any(feature = "std", test)), no_std)]

/// secrets module is used by both the ectf25_design package and build scripts
#[cfg(feature = "std")]
pub mod secrets;

pub const MAX_STANDARD_CHANNEL: u32 = 8;

#[derive(Debug)]
pub struct Subscription {
    channel_id: u32,
    start: u64,
    end: u64
}

#[derive(Debug)]
pub struct Channels([Option<Subscription>; 8]);

#[derive(Debug)]
pub struct EncodedFrame<'a> {
    data: &'a [u8]
}

#[derive(Debug)]
pub struct PartiallyDecodedFrame<'a> {
    frame_data: &'a [u8],
    channel_id: u32,
    timestamp: u64
}

#[derive(Debug)]
pub struct DecodedFrame<'a> {
    frame: &'a [u8]
}

impl Default for Subscription {
    fn default() -> Self {
        Subscription {
            channel_id: 0xFFFFFFFF,
            start: 0x0,
            end: 0x0
        }
    }
}

pub trait BytesSerializable<const L: usize> {
    fn to_bytes(&self) -> [u8; L];
    fn from_bytes(bytes: [u8; L]) -> Self;
}

/// Follows the format:
/// \[32-bit channel ID]\[64-bit start timestamp]\[64-bit end timestamp]
/// or:
/// \[4 bytes]\[8 bytes]\[8 bytes]
impl BytesSerializable<20> for Subscription {
    fn to_bytes(&self) -> [u8; 20] {
        let mut bytes = [0; 20];

        bytes[..4].copy_from_slice(&self.channel_id.to_be_bytes());
        bytes[4..12].copy_from_slice(&self.start.to_be_bytes());
        bytes[12..].copy_from_slice(&self.end.to_be_bytes());

        bytes
    }

    fn from_bytes(bytes: [u8; 20]) -> Subscription {
        let channel_id = u32::from_be_bytes(bytes[..4].try_into().unwrap());
        let start = u64::from_be_bytes(bytes[4..12].try_into().unwrap());
        let end = u64::from_be_bytes(bytes[12..].try_into().unwrap());

        Subscription {
            channel_id,
            start,
            end
        }
    }
}

impl BytesSerializable<161> for Channels {
    fn to_bytes(&self) -> [u8; 161] {
        let mut bytes = [0; 161];

        // The first byte is a bitmask that indicates which channels are present
        let mut bitmask = 0;
        for i in 0..8 {
            if self.0[i].is_some() {
                bitmask |= 1 << i;
            }
        }
        bytes[0] = bitmask;

        // The rest of the bytes are the serialized channels (active subscriptions)
        let mut offset = 1;
        for i in 0..8 {
            if let Some(sub) = self.0[i].as_ref() {
                bytes[offset..offset + 20].copy_from_slice(&sub.to_bytes());
            }
            offset += 20;
        }

        bytes
    }

    fn from_bytes(bytes: [u8; 161]) -> Self {
        let mut channels = [const{ None }; 8];

        let bitmask = bytes[0];
        let mut offset = 1;
        for i in 0..8 {
            if bitmask & (1 << i) != 0 {
                channels[i] = Some(Subscription::from_bytes(bytes[offset..offset + 20].try_into().unwrap()));
            }
            offset += 20;
        }

        Channels(channels)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_subscription_to_bytes() {
        let sub = Subscription {
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
    fn test_channels_to_bytes() {
        let sub1 = Subscription {
            channel_id: 0x1,
            start: 0x32,
            end: 0x1F4
        };

        let sub2 = Subscription {
            channel_id: 0x02,
            start: 0x32,
            end: 0x1F4
        };

        let mut channels_bytes: [u8; 161] = [0; 161];
        channels_bytes[0] = 0b11;
        channels_bytes[1..21].copy_from_slice(&sub1.to_bytes());
        channels_bytes[21..41].copy_from_slice(&sub2.to_bytes());

        let channels = Channels([Some(sub1), Some(sub2), None, None, None, None, None, None]);
        assert_eq!(channels.to_bytes(), channels_bytes);
    }
}
