#![cfg_attr(not(test), no_std)]

pub static MAX_CHANNEL_COUNT: u8 = 8;
pub static EMERGENCY_CHANNEL: u8 = 0;
pub static FRAME_SIZE: u8 = 64;
pub static DEFAULT_CHANNEL_TIMESTAMP: u32 = 0xFFFFFFFF;

#[derive(Debug)]
pub struct Subscription {
    channel_id: u32,
    start: u64,
    end: u64
}

#[derive(Debug)]
pub struct Channels {
    // Note: Channel 0 does not need a subscription
    channel_1: Option<Subscription>,
    channel_2: Option<Subscription>,
    channel_3: Option<Subscription>,
    channel_4: Option<Subscription>,
    channel_5: Option<Subscription>,
    channel_6: Option<Subscription>,
    channel_7: Option<Subscription>,
    channel_8: Option<Subscription>
}

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
            channel_id: 0,
            start: 0,
            end: 0
        }
    }
}

impl Channels {
    fn update_subscriptions(&mut self, subscription: Subscription) -> () {
        match subscription.channel_id {
            0 => (),
            1 => self.channel_1 = Some(subscription),
            2 => self.channel_2 = Some(subscription),
            3 => self.channel_3 = Some(subscription),
            4 => self.channel_4 = Some(subscription),
            5 => self.channel_5 = Some(subscription),
            6 => self.channel_6 = Some(subscription),
            7 => self.channel_7 = Some(subscription),
            8 => self.channel_8 = Some(subscription),
            _ => ()
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

// impl BytesSerializable<160> for Channels {
//     fn to_bytes(&self) -> [u8; 160] {
//         let mut bytes = [0; 160];
//
//         bytes[..20].copy_from_slice(&self.channel_1.unwrap_or_default().to_bytes());
//
//         bytes
//     }
//
//     fn from_bytes(bytes: [u8; 160]) -> Self {
//         let channel_1 = Subscription::from_bytes(bytes[..20].try_into().unwrap());
//         let channel_2 = Subscription::from_bytes(bytes[20..40].try_into().unwrap());
//         let channel_3 = Subscription::from_bytes(bytes[40..60].try_into().unwrap());
//         let channel_4 = Subscription::from_bytes(bytes[60..80].try_into().unwrap());
//         let channel_5 = Subscription::from_bytes(bytes[80..100].try_into().unwrap());
//         let channel_6 = Subscription::from_bytes(bytes[100..120].try_into().unwrap());
//         let channel_7 = Subscription::from_bytes(bytes[120..140].try_into().unwrap());
//         let channel_8 = Subscription::from_bytes(bytes[140..].try_into().unwrap());
//
//         Channels {
//             channel_1: Some(channel_1),
//             channel_2: Some(channel_2),
//             channel_3: Some(channel_3),
//             channel_4: Some(channel_4),
//             channel_5: Some(channel_5),
//             channel_6: Some(channel_6),
//             channel_7: Some(channel_7),
//             channel_8: Some(channel_8)
//         }
//     }
// }


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_subscription_to_bytes() {
        let sub = Subscription {
            channel_id: 1,
            start: 50,
            end: 500
        };
        let sub_bytes: [u8; 20] = [
            0, 0, 0, 1,
            0, 0, 0, 0, 0, 0, 0, 50,
            0, 0, 0, 0, 0, 0, 1, 244
        ];
        assert_eq!(sub.to_bytes(), sub_bytes);
    }

    #[test]
    fn test_channels_to_bytes() {
        let sub1 = Subscription {
            channel_id: 1,
            start: 50,
            end: 500
        };

        let sub2 = Subscription {
            channel_id: 2,
            start: 100,
            end: 1000
        };

        let channels = Channels {
            channel_1: Some(sub1),
            channel_2: Some(sub2),
            channel_3: None,
            channel_4: None,
            channel_5: None,
            channel_6: None,
            channel_7: None,
            channel_8: None
        };

        // println!("{:?}", channels.to_bytes());
    }
}
