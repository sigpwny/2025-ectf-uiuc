use ectf_board::{
    Board,
    ectf_constants::{*}
};
use ascon_aead::{Ascon128, Key, Nonce}; // Or `Ascon128a`
use ascon_aead::aead::{Aead, KeyInit};

use common::{SubscriptionInfo, SubscriptionInfoList, Picture, DecryptedFrame, EncryptedFrame, EncryptedPicture, BytesSerializable};
use max7800x_hal::flc::{Flc, FlashError, FLASH_BASE, FLASH_PAGE_SIZE};

use crate::constants::*;
use crate::{
    ChannelSecret, PictureKey,
};
use tiny_keccak::{Hasher, Kmac};

pub const FLASH_ADDR_SUBSCRIPTION_BASE: u32 = FLASH_BASE + (27 * FLASH_PAGE_SIZE);

pub static MAX_CHANNEL_COUNT: u8 = 8;
pub static EMERGENCY_CHANNEL: u8 = 0;
pub static FRAME_SIZE: u8 = 64;
pub static DEFAULT_CHANNEL_TIMESTAMP: u32 = 0xFFFFFFFFFFFFFFFF;
pub static FLASH_FIRST_BOOT: u32 = 0xDEADBEEF; // arbitrary canary value 





// Type Definitions

struct LastTime {
    time: u64,
}

// Utility Functions

fn get_channel_secret(channel_id: u32, timestamp: u64, flc: &mut Flc) -> Option<ChannelSecret> {
    let stored_sub = get_channel_subscription(flc, channel_id);
    let sub_info = stored_sub.info;

    if (timestamp >= info.start  && timestamp <= info.end) {
        Some(ChannelSecret {
            stored_sub.channel_secret
        })
    } else {
        None
    }
}

//Core Functions

fn decode_frame(request: &EncyptedFrame, last_time: &mut LastTime, flc: &mut Flc) -> Picture {
    let response_packet: Picture();
    let frame_key = Key::<Ascon128>::from_slice(flc.read_128(0x1003_4000));
    let frame_cipher = Ascon128::new(frame_key);
    let partially_decoded =  DecryptedFrame.from_bytes(frame_cipher.decrypt(request).expect("decryption failure!"));


    if partially_decoded.timestamp > last_time.time { /*last_time will be an instance of a LastTime struct with time intialized to zero*/
        let channel_secret = get_channel_secret(partially_decoded.channel_id, partially_decoded.timestamp, flc);
        let picture_key = Key::<Ascon128>::from_slice(derive_picture_key(channel_secret, partially_decoded.timestamp));
        match channel_secret {
            Some(secret) => {
                response_packet = Picture.from_bytes(Ascon128::new(picture_key).decrypt(partially_decoded.encrypted_picture).expect("decryption failure!"));
                last_time = LastTime{time:partially_decoded.timestamp};
                return response_packet;
            },
            None => print_error("Channel Subscription is not present"),
        };
            
    } else {
        print_error("Invalid Timestamp");
    }

    return response_packet;
}