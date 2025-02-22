use common::{SubscriptionInfo, SubscriptionInfoList};
use max7800x_hal::flc::{Flc, FlashError, FLASH_BASE, FLASH_PAGE_SIZE}

pub const FLASH_ADDR_SUBSCRIPTION_BASE: u32 = FLASH_BASE + (27 * FLASH_PAGE_SIZE);

// assert_eq!(FLASH_ADDR_SUBSCRIPTION_BASE, 0x10060000);

// 0x1000_0000
// firmware size = 0x38000
// 0x1003_4000 - 0x1003_5FFF - global secrets
//  - frame key (16 bytes)
//  - subscription key (16 bytes, specific for decoder)

// 0x1003_6000 - 0x1003_7FFF (size: 0x2000, which is 8192 bytes) - subscription storage
// index 27
// channel 0 subscription (valid for 0x0 - max timestamp (u64))
// each subscription: 16 bytes (0x53) + 8 bytes (start) + 8 bytes (end) + 32 bytes (channel secret)

// 0x1008_0000 (64 pages total)

// https://docs.rs/max7800x-hal/latest/max7800x_hal/flc/struct.Flc.html


// This is only called after we have verified/authenticated/decrypted the update subscription message
pub fn update_subscription(flc: &mut Flc, subscription: StoredSubscriptionList) -> Result<(), FlashError> {
    unsafe {
        flc.erase_page(FLASH_ADDR_SUBSCRIPTION_BASE)?;
    }
    // let _ = flc.write_128(...)?;
}

// For the decoder function
pub fn get_channel_subscription(flc: &mut Flc, channel_id: u32) -> Result<StoredSubscription, ()> {
    // Ensure channel ID is 0-8 (inclusive)
    if channel_id < 0 || channel_id > 8 {
        return Ok(());
    }
    
    let subscription_loc: u32 = FLASH_ADDR_SUBSCRIPTION_BASE + channel_id * 64;
    
    // make sure to validate first 16 bytes are 0x53 
    let header: [u32; 4] = flc.read_128(subscription_loc).unwrap();
    for i in 0..4 {
        if header[i] != 0x5353_5353 {
            return Ok(());
        }
    }

    let timestamps: [u32; 4] = flc.read_128(subscription_loc + 64).unwrap();
    let start: u64 = ((timestamps[0] as u64) << 32) + (timestamps[1] as u64); // could be a catastrophic bit fiddling error
    let end: u64 = ((timestamps[2] as u64) << 32) + (timestamps[3] as u64);   // here too
    let info: SubscriptionInfo = SubscriptionInfo {
        channel_id,
        start,
        end
    }

    let sec_p1: [u32; 4] = flc.read_128(subscription_loc + 64*2).unwrap();
    let sec_p2: [u32; 4] = flc.read_128(subscription_loc + 64*3).unwrap();
    let channel_secret: [u8; 32] = [0; 32];
    for i in 0..8 {
        let b1: u8 = if i < 4 { ((sec_p1[i] & 0xFF00_0000) >> 24) as u8 } else { ((sec_p2[4 - i] & 0xFF00_0000) >> 24) as u8 }; // this could also be a bit fiddling tragedy
        let b2: u8 = if i < 4 { ((sec_p1[i] & 0x00FF_0000) >> 16) as u8 } else { ((sec_p2[4 - i] & 0x00FF_0000) >> 16) as u8 };
        let b3: u8 = if i < 4 { ((sec_p1[i] & 0x0000_FF00) >>  8) as u8 } else { ((sec_p2[4 - i] & 0x0000_FF00) >>  8) as u8 };
        let b4: u8 = if i < 4 { ((sec_p1[i] & 0x0000_00FF) >>  0) as u8 } else { ((sec_p2[4 - i] & 0x0000_00FF) >>  0) as u8 };

        channel_secret[4*i] = b1;
        channel_secret[4*i + 1] = b2;
        channel_secret[4*i + 2] = b3;
        channel_secret[4*i + 3] = b4;
    }

    return Ok(StoredSubscription {
        info,
        channel_secret
    })
}

// For list subscriptions
pub fn get_subscriptions(flc: &mut Flc) -> SubscriptionInfoList {
    // call get_channel_subscription_info for each channel (1-8)
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