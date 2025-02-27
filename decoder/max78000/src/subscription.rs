use crate::crypto::{decrypt_ascon, get_subscription_key};
use crate::hal::flc::{FlashError, Flc};
use bincode::decode_from_slice;
use common::constants::*;
use common::{
    check_complement_16b, make_complement_16b, ChannelSecret, EncryptedSubscription,
    StoredSubscription, SubscriptionInfo, SubscriptionInfoList, BINCODE_CONFIG,
};
use zeroize::Zeroize;

// Everything is 16B aligned. Every 16B is complemented by the next 16B.
// ┌───────────────────────────┐
// │Channel Subscription       │
// ├───────────────────────────┤
// │Magic (4B), Chan. ID (4B)  │
// │Magic (4B), Chan. ID (4B)  │
// │~Magic (4B), ~Chan. ID (4B)│
// │~Magic (4B), ~Chan. ID (4B)│
// │Start Timestamp (8B)       │
// │End Timestamp (8B)         │
// │~Start Timestamp (8B)      │
// │~End Timestamp (8B)        │
// │Channel Secret 1/2 (16B)   │
// │~Channel Secret 1/2 (16B)  │
// │Channel Secret 2/2 (16B)   │
// │~Channel Secret 2/2 (16B)  │
// └───────────────────────────┘

/// Helper function to write 16 bytes to flash
pub fn write_16b(flc: &mut Flc, addr: u32, data: &[u8; 16]) -> Result<(), FlashError> {
    let data_u32: [u32; 4] = [
        u32::from_le_bytes(data[0..4].try_into().unwrap()),
        u32::from_le_bytes(data[4..8].try_into().unwrap()),
        u32::from_le_bytes(data[8..12].try_into().unwrap()),
        u32::from_le_bytes(data[12..16].try_into().unwrap()),
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

/// Decrypts the subscription and returns a StoredSubscription.
pub fn decrypt_subscription(
    enc_subscription: EncryptedSubscription,
) -> Result<StoredSubscription, ()> {
    let mut dec_sub_bytes = [0u8; LEN_STORED_SUBSCRIPTION];

    let mut subscription_key = get_subscription_key();
    match decrypt_ascon(&enc_subscription.0, &subscription_key.0, &mut dec_sub_bytes) {
        Ok(_) => (),
        Err(_) => return Err(()),
    }
    subscription_key.zeroize();
    let dec_sub: StoredSubscription = match decode_from_slice(&dec_sub_bytes, BINCODE_CONFIG) {
        Ok((sub, LEN_STORED_SUBSCRIPTION)) => sub,
        _ => return Err(()),
    };
    Ok(dec_sub)
}

/// Updates the given subscription in flash memory.
/// - Iterates through the available flash pages.
/// - If a subscription is found with the same channel ID, it is replaced.
/// - If an empty or invalid subscription is found, the new subscription is written.
/// - If there are no more slots available, the subscription is not written and an error is returned.
pub fn update_subscription(flc: &mut Flc, new_sub: StoredSubscription) -> Result<(), FlashError> {
    assert!(
        new_sub.info.channel_id != EMERGENCY_CHANNEL_ID,
        "Invalid channel ID"
    );

    for idx in 1..=LEN_STANDARD_CHANNELS as u32 {
        match get_subscription_at_idx(flc, idx) {
            Ok(sub) => {
                // If the channel ID matches, replace the subscription
                if sub.info.channel_id == new_sub.info.channel_id {
                    return write_subscription(flc, idx, new_sub);
                }
            }
            Err(_) => {
                // If the subscription is invalid, write the new subscription
                return write_subscription(flc, idx, new_sub);
            }
        }
    }

    // If we get here, there are no more slots available
    Err(FlashError::NeedsErase)
}

/// Writes the given subscription to flash memory at the given index.
fn write_subscription(
    flc: &mut Flc,
    idx: u32,
    new_sub: StoredSubscription,
) -> Result<(), FlashError> {
    let sub_addr: u32 = FLASH_ADDR_SUBSCRIPTION_BASE + (idx * FLASH_PAGE_SIZE);

    unsafe {
        flc.erase_page(sub_addr)?;
    }

    // Write the header
    let mut header_bytes = [FLASH_MAGIC_SUBSCRIPTION; 16];
    header_bytes[4..8].copy_from_slice(&new_sub.info.channel_id.to_le_bytes());
    header_bytes[12..16].copy_from_slice(&new_sub.info.channel_id.to_le_bytes());
    write_16b(flc, sub_addr, &header_bytes)?;
    write_16b(flc, sub_addr + 16, &make_complement_16b(&header_bytes))?;

    // Write the timestamps
    let mut timestamp_bytes = [0u8; 16];
    timestamp_bytes[0..8].copy_from_slice(&new_sub.info.start.to_le_bytes());
    timestamp_bytes[8..16].copy_from_slice(&new_sub.info.end.to_le_bytes());
    write_16b(flc, sub_addr + 32, &timestamp_bytes)?;
    write_16b(flc, sub_addr + 48, &make_complement_16b(&timestamp_bytes))?;

    // Write the channel secret
    let mut channel_secret_bytes_1 = [0u8; 16];
    channel_secret_bytes_1.copy_from_slice(&new_sub.channel_secret.0[0..16]);
    write_16b(flc, sub_addr + 64, &channel_secret_bytes_1)?;
    write_16b(
        flc,
        sub_addr + 80,
        &make_complement_16b(&channel_secret_bytes_1),
    )?;
    channel_secret_bytes_1.zeroize();
    let mut channel_secret_bytes_2 = [0u8; 16];
    channel_secret_bytes_2.copy_from_slice(&new_sub.channel_secret.0[16..32]);
    write_16b(flc, sub_addr + 96, &channel_secret_bytes_2)?;
    write_16b(
        flc,
        sub_addr + 112,
        &make_complement_16b(&channel_secret_bytes_2),
    )?;
    channel_secret_bytes_2.zeroize();

    Ok(())
}

/// Gets the subscription at the given index in flash.
/// Performs integrity checks on the stored subscription to ensure it is valid.
pub fn get_subscription_at_idx(flc: &mut Flc, idx: u32) -> Result<StoredSubscription, ()> {
    if idx as usize > LEN_STANDARD_CHANNELS {
        return Err(());
    }

    let sub_addr: u32 = FLASH_ADDR_SUBSCRIPTION_BASE + (idx * FLASH_PAGE_SIZE);

    // Shared complement bytes
    let mut complement_bytes = [0u8; 16];

    // Validate magic bytes, channel ID, magic bytes, channel ID
    let mut header_bytes = [0u8; 16];
    read_16b(flc, sub_addr, &mut header_bytes).unwrap();
    read_16b(flc, sub_addr + 16, &mut complement_bytes).unwrap();
    if !check_complement_16b(&header_bytes, &complement_bytes) {
        return Err(());
    }
    // Check magic bytes
    if header_bytes[0..4] != [FLASH_MAGIC_SUBSCRIPTION; 4] {
        return Err(());
    }
    if header_bytes[8..12] != [FLASH_MAGIC_SUBSCRIPTION; 4] {
        return Err(());
    }
    // Check channel ID
    let channel_id: u32 = u32::from_le_bytes(header_bytes[4..8].try_into().unwrap());
    let channel_id_temp: u32 = u32::from_le_bytes(header_bytes[12..16].try_into().unwrap());
    if channel_id != channel_id_temp {
        return Err(());
    }

    // Read the timestamps
    let mut timestamp_bytes = [0u8; 16];
    read_16b(flc, sub_addr + 32, &mut timestamp_bytes).unwrap();
    read_16b(flc, sub_addr + 48, &mut complement_bytes).unwrap();
    if !check_complement_16b(&timestamp_bytes, &complement_bytes) {
        return Err(());
    }
    let start: u64 = u64::from_le_bytes(timestamp_bytes[0..8].try_into().unwrap());
    let end: u64 = u64::from_le_bytes(timestamp_bytes[8..16].try_into().unwrap());
    if start > end {
        return Err(());
    }

    // Read the channel secret
    let mut channel_secret_bytes_1 = [0u8; 16];
    read_16b(flc, sub_addr + 64, &mut channel_secret_bytes_1).unwrap();
    read_16b(flc, sub_addr + 80, &mut complement_bytes).unwrap();
    if !check_complement_16b(&channel_secret_bytes_1, &complement_bytes) {
        return Err(());
    }
    let mut channel_secret_bytes_2 = [0u8; 16];
    read_16b(flc, sub_addr + 96, &mut channel_secret_bytes_2).unwrap();
    read_16b(flc, sub_addr + 112, &mut complement_bytes).unwrap();
    if !check_complement_16b(&channel_secret_bytes_2, &complement_bytes) {
        return Err(());
    }

    complement_bytes.zeroize();

    let mut channel_secret_bytes = [0u8; 32];
    channel_secret_bytes[0..16].copy_from_slice(&channel_secret_bytes_1);
    channel_secret_bytes[16..32].copy_from_slice(&channel_secret_bytes_2);

    channel_secret_bytes_1.zeroize();
    channel_secret_bytes_2.zeroize();

    // Construct the stored subscription
    let stored_sub = StoredSubscription {
        info: SubscriptionInfo {
            channel_id,
            start,
            end,
        },
        channel_secret: ChannelSecret(channel_secret_bytes),
    };

    Ok(stored_sub)
}

/// Gets the subscription for the given channel ID.
pub fn get_channel_subscription(flc: &mut Flc, channel_id: u32) -> Result<StoredSubscription, ()> {
    for idx in 0..=LEN_STANDARD_CHANNELS as u32 {
        match get_subscription_at_idx(flc, idx) {
            Ok(sub) => {
                if sub.info.channel_id == channel_id {
                    return Ok(sub);
                }
            }
            Err(_) => (),
        }
    }

    Err(())
}

/// Returns a list of all valid subscriptions in flash.
pub fn list_subscriptions(flc: &mut Flc) -> SubscriptionInfoList {
    let mut subscriptions = core::array::from_fn(|_| SubscriptionInfo {
        channel_id: 0,
        start: 0,
        end: 0,
    });

    let mut num_sub_channels: usize = 0;
    for idx in 1..=LEN_STANDARD_CHANNELS as u32 {
        match get_subscription_at_idx(flc, idx) {
            Ok(sub) => {
                subscriptions[num_sub_channels] = sub.info;
                num_sub_channels += 1;
            }
            Err(_) => (),
        }
    }

    SubscriptionInfoList {
        num_sub_channels: num_sub_channels as u32,
        subscriptions,
    }
}
