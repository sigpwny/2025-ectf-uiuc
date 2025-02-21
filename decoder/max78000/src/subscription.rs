use common::{SubscriptionInfo, SubscriptionInfoList};
use max7800x_hal::flc::{Flc, FLASH_BASE, FLASH_PAGE_SIZE}

pub const FLASH_ADDR_SUBSCRIPTION_BASE: usize = FLASH_BASE + (48 * FLASH_PAGE_SIZE);

// assert_eq!(FLASH_ADDR_SUBSCRIPTION_BASE, 0x10060000);

// 0x1000_0000
// firmware size = 0x38000
// 0x1003_4000 - 0x1003_5FFF - global secrets
//  - frame key (16 bytes)
//  - subscription key (16 bytes, specific for decoder)

// 0x1003_6000 - 0x1003_7FFF (size: 0x2000, which is 8192 bytes) - subscription storage
// index 27
// channel 0 subscription (valid for 0x0 - max timestamp (u64))
// each subscription: 8 bytes (start) + 8 bytes (end) + 16 bytes (0x00) + 32 bytes (channel secret)

// 0x1008_0000 (64 pages total)

// https://docs.rs/max7800x-hal/latest/max7800x_hal/flc/struct.Flc.html



pub fn update_subscription(flc: &mut Flc, ????) {

}

// For the decoder function
pub fn get_channel_subscription_info(flc: &mut Flc, channel_id: u32) -> SubscriptionInfo {
    // Ensure channel ID is 0-8 (inclusive)
}

// For list subscriptions
pub fn get_subscriptions(flc: &mut Flc) -> SubscriptionInfoList {
    // call get_channel_subscription_info for each channel
}

/*
CHANNEL_ADDRESSES = [u32; 8] //flash addresses for each subscription

fn as_u32_le(array: &[u8; 4]) -> u32 {
    ((array[0] as u32) <<  0) +
    ((array[1] as u32) <<  8) +
    ((array[2] as u32) << 16) +
    ((array[3] as u32) << 24)
}

store_subscription(Channels channels):
    // assume channels is already verified at this stage
    for i in 0..channels.len():
        subscription_bytes = channels[i].Some().to_bytes();
        32bit_sub = as_32_le(subscription_bytes);
        flc.write_32(CHANNEL_ADDRESS[i], 32bit_sub);
    return write_to_flash_message;

read_from_flash(int channel_num):
    subscription = flc.read_32(CHANNEL_ADDRESSES[i]).unwrap();
    if subscription == {default_value}:
        return Err;
    else:
        return subscription;
*/