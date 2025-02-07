#![no_std]

pub static MAX_CHANNEL_COUNT: u8 = 8;
pub static EMERGENCY_CHANNEL: u8 = 0;
pub static FRAME_SIZE: u8 = 64;
pub static DEFAULT_CHANNEL_TIMESTAMP: u32 = 0xFFFFFFFFFFFFFFFF;


struct Channels {
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

struct Subscription {
    channel_id: u32,
    start: u64,
    end: u64
}


struct EncodedFrame<'a> {
    data: &'a [u8]
}

struct PartiallyDecodedFrame<'a> {
    frame_data: &'a [u8],
    channel_id: u32,
    timestamp: u64
}

struct DecodedFrame<'a> {
    frame: &'a [u8]
}
