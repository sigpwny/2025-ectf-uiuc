use common::{
    ChannelSecret,
    SubscriptionInfo,
    SubscriptionInfoList,
    StoredSubscription,
};
use common::constants::*;
use max7800x_hal::flc::{Flc, FlashError};
use zeroize::Zeroize;

/// Helper function to write 16 bytes to flash
pub fn write_16b(flc: &mut Flc, addr: u32, data: &[u8; 16]) -> Result<(), FlashError> {
    let data_u32: [u32; 4] = [
        u32::from_le_bytes([data[3], data[2], data[1], data[0]]),
        u32::from_le_bytes([data[7], data[6], data[5], data[4]]),
        u32::from_le_bytes([data[11], data[10], data[9], data[8]]),
        u32::from_le_bytes([data[15], data[14], data[13], data[12]]),
    ];

    flc.write_128(addr, &data_u32)
}

/// Helper function to read 16 bytes from flash
pub fn read_16b(flc: &mut Flc, addr: u32, data: &mut [u8; 16]) -> Result<(), FlashError> {
    let data_u32: [u32; 4] = flc.read_128(addr)?;

    for i in 0..4 {
        let start = i * 4;
        let end = start + 4;
        let chunk = &data_u32[i].to_le_bytes();

        data[start..end].copy_from_slice(chunk);
    }

    Ok(())
}

// This is only called after we have verified/authenticated/decrypted the update subscription message
pub fn update_subscription(flc: &mut Flc, new_sub: StoredSubscription) -> Result<(), FlashError> {
    assert!(new_sub.info.channel_id >= 1 && new_sub.info.channel_id <= MAX_STANDARD_CHANNEL, "Invalid channel ID");

    let sub_addr: u32 = FLASH_ADDR_SUBSCRIPTION_BASE + (new_sub.info.channel_id * FLASH_PAGE_SIZE);

    unsafe {
        flc.erase_page(sub_addr)?;
    }

    // The first 16 bytes are magic bytes and remain unchanged
    let mut sub_bytes = [0x53u8; 64];

    // Add the timestamps
    let start_bytes = new_sub.info.start.to_le_bytes();
    let end_bytes = new_sub.info.end.to_le_bytes();
    sub_bytes[16..24].copy_from_slice(&start_bytes);
    sub_bytes[24..32].copy_from_slice(&end_bytes);

    // Add the channel secret
    sub_bytes[32..64].copy_from_slice(&new_sub.channel_secret.0);

    // Write 64 bytes (4x16) to flash
    for i in 0..4 {
        let start = i * 16;
        let end = start + 16;
        let mut chunk: [u8; 16] = sub_bytes[start..end].try_into().unwrap();

        write_16b(flc, sub_addr + (i * 16) as u32, &chunk)?;
        chunk.zeroize();
    }

    sub_bytes.zeroize();

    Ok(())
}

// For the decoder function
pub fn get_channel_subscription(flc: &mut Flc, channel_id: u32) -> Result<StoredSubscription, ()> {
    // Ensure channel ID is 0-8 (inclusive)
    if channel_id > MAX_STANDARD_CHANNEL {
        return Err(());
    }

    let sub_addr: u32 = FLASH_ADDR_SUBSCRIPTION_BASE + (channel_id * FLASH_PAGE_SIZE);

    // Validate magic bytes (indicates an enabled subscription)
    let mut header_bytes = [0u8; 16];
    read_16b(flc, sub_addr, &mut header_bytes).unwrap();
    for byte in &header_bytes {
        if *byte != 0x53 {
            return Err(());
        }
    }

    // Read the timestamps
    let mut timestamp_bytes = [0u8; 16];
    read_16b(flc, sub_addr + 16, &mut timestamp_bytes).unwrap();
    let start: u64 = u64::from_le_bytes(timestamp_bytes[0..8].try_into().unwrap());
    let end: u64 = u64::from_le_bytes(timestamp_bytes[8..16].try_into().unwrap());
    assert!(start <= end, "Invalid subscription timestamps");

    // Read the channel secret
    let mut channel_secret_bytes_1 = [0u8; 16];
    let mut channel_secret_bytes_2 = [0u8; 16];
    read_16b(flc, sub_addr + 32, &mut channel_secret_bytes_1).unwrap();
    read_16b(flc, sub_addr + 48, &mut channel_secret_bytes_2).unwrap();
    let mut channel_secret_bytes = [0u8; 32];
    channel_secret_bytes[0..16].copy_from_slice(&channel_secret_bytes_1);
    channel_secret_bytes[16..32].copy_from_slice(&channel_secret_bytes_2);

    // Construct the stored subscription
    let stored_sub = StoredSubscription {
        info: SubscriptionInfo {
            channel_id,
            start,
            end,
        },
        channel_secret: ChannelSecret(channel_secret_bytes),
    };

    // Zeroize the temporary variables for channel secret
    channel_secret_bytes_1.zeroize();
    channel_secret_bytes_2.zeroize();

    Ok(stored_sub)
}

// For list subscriptions
pub fn get_subscriptions(flc: &mut Flc) -> SubscriptionInfoList {
    let mut subscriptions = core::array::from_fn(|_| SubscriptionInfo {
        channel_id: 0,
        start: 0,
        end: 0,
    });

    let mut subscribed_channels: usize = 0;
    for channel_id in 1..=MAX_STANDARD_CHANNEL {
        match get_channel_subscription(flc, channel_id) {
            Ok(sub) => {
                subscriptions[subscribed_channels] = sub.info;
                subscribed_channels += 1;
            }
            Err(_) => (),
        }
    }

    SubscriptionInfoList {
        subscribed_channels: subscribed_channels as u32,
        subscriptions,
    }
}
