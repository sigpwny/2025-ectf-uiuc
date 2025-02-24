use crate::constants::*;
use crate::{
    BaseChannelSecret, BaseSubscriptionSecret, ChannelSecret, PictureKey, SubscriptionKey,
};
use tiny_keccak::{Hasher, Kmac};

pub fn derive_channel_secret(
    base_channel_secret: &BaseChannelSecret,
    channel_id: u32,
) -> ChannelSecret {
    let mut kmac = Kmac::v256(&base_channel_secret.0, b"derive_channel_secret");
    kmac.update(&channel_id.to_le_bytes());
    let mut channel_secret = [0u8; LEN_CHANNEL_SECRET];
    kmac.finalize(&mut channel_secret);
    ChannelSecret(channel_secret)
}

pub fn derive_subscription_key(
    base_subscription_secret: &BaseSubscriptionSecret,
    decoder_id: u32,
) -> SubscriptionKey {
    let mut kmac = Kmac::v128(&base_subscription_secret.0, b"derive_subscription_key");
    kmac.update(&decoder_id.to_le_bytes());
    let mut subscription_key = [0u8; LEN_ASCON_KEY];
    kmac.finalize(&mut subscription_key);
    SubscriptionKey(subscription_key)
}

pub fn derive_picture_key(channel_secret: &ChannelSecret, timestamp: u64) -> PictureKey {
    let mut kmac = Kmac::v128(&channel_secret.0, b"derive_picture_key");
    kmac.update(&timestamp.to_le_bytes());
    kmac.update(&(!timestamp).to_le_bytes());
    let mut picture_key = [0u8; LEN_ASCON_KEY];
    kmac.finalize(&mut picture_key);
    PictureKey(picture_key)
}
