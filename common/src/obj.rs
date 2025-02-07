#![no_std]

pub static MAX_CHANNEL_COUNT: u8 = 8;
pub static EMERGENCY_CHANNEL: u8 = 0;
pub static FRAME_SIZE: u8 = 64;
pub static DEFAULT_CHANNEL_TIMESTAMP: u32 = 0xFFFFFFFFFFFFFFFF;


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

pub struct Subscription {
    channel_id: u32,
    start: u64,
    end: u64
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
