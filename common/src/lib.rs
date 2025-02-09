#![no_std]

pub static MAX_CHANNEL_COUNT: u8 = 8;
pub static EMERGENCY_CHANNEL: u8 = 0;
pub static FRAME_SIZE: u8 = 64;
pub static DEFAULT_CHANNEL_TIMESTAMP: u32 = 0xFFFFFFFFFFFFFFFF;

pub struct Subscription {
    channel_id: u32,
    start: u64,
    end: u64
}

pub struct Channels<'a> {
    // Note: Channel 0 does not need a subscription
    channel_1: Option<&'a Subscription>,
    channel_2: Option<&'a Subscription>,
    channel_3: Option<&'a Subscription>,
    channel_4: Option<&'a Subscription>,
    channel_5: Option<&'a Subscription>,
    channel_6: Option<&'a Subscription>,
    channel_7: Option<&'a Subscription>,
    channel_8: Option<&'a Subscription>
}

pub struct EncodedFrame<'a> {
    data: &'a [u8]
}

pub struct PartiallyDecodedFrame<'a> {
    frame_data: &'a [u8],
    channel_id: u32,
    timestamp: u64
}

pub struct DecodedFrame<'a> {
    frame: &'a [u8]
}

impl<'a> Channels<'a> {
    fn update_subscriptions(&mut self, subscription: &Subscription) -> () {
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

pub trait BytesSerializable {
    fn to_bytes(&self) -> [u8];
    fn from_bytes(bytes: [u8]) -> Self;
}

/// Follows the format:
/// \[32-bit channel ID]\[64-bit start timestamp]\[64-bit end timestamp]
/// or:
/// \[4 bytes]\[8 bytes]\[8 bytes]
impl BytesSerializable for Subscription {
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

impl BytesSerializable for Channels {
    fn to_bytes(&self) -> [u8; 160] {
        let mut bytes = [0; 160];

        if let Some(subscription) = self.channel_1 {
            bytes[..20].copy_from_slice(&subscription.to_bytes());
        }
        if let Some(subscription) = self.channel_2 {
            bytes[20..40].copy_from_slice(&subscription.to_bytes());
        }
        if let Some(subscription) = self.channel_3 {
            bytes[40..60].copy_from_slice(&subscription.to_bytes());
        }
        if let Some(subscription) = self.channel_4 {
            bytes[60..80].copy_from_slice(&subscription.to_bytes());
        }
        if let Some(subscription) = self.channel_5 {
            bytes[80..100].copy_from_slice(&subscription.to_bytes());
        }
        if let Some(subscription) = self.channel_6 {
            bytes[100..120].copy_from_slice(&subscription.to_bytes());
        }
        if let Some(subscription) = self.channel_7 {
            bytes[120..140].copy_from_slice(&subscription.to_bytes());
        }
        if let Some(subscription) = self.channel_8 {
            bytes[140..].copy_from_slice(&subscription.to_bytes());
        }

        bytes
    }

    fn from_bytes(bytes: [u8; 160]) -> Self {
        !todo!("This probably won't be possible unless Channels is changed to own its Subscriptions")
    }
}
