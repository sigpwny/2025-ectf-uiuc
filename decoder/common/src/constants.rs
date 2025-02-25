// Primitives
pub const LEN_CHANNEL_ID: usize = 4;
pub const LEN_TIMESTAMP: usize = 8;
pub const MAX_STANDARD_CHANNEL: u32 = 8;

pub const LEN_RNG_SEED: usize = 64;

// Ascon constants
pub const LEN_ASCON_KEY: usize = 16;
pub const LEN_ASCON_NONCE: usize = 16;
pub const LEN_ASCON_TAG: usize = 16;
pub const LEN_ASCON_AEAD_OVERHEAD: usize = LEN_ASCON_NONCE + LEN_ASCON_TAG;

// Secrets constants
pub const LEN_BASE_CHANNEL_SECRET: usize = 32;
pub const LEN_BASE_SUBSCRIPTION_SECRET: usize = 32;
pub const LEN_CHANNEL_SECRET: usize = 32;

// Update subscription constants
pub const LEN_SUBSCRIPTION_INFO: usize = LEN_CHANNEL_ID + 2 * LEN_TIMESTAMP;
pub const LEN_STORED_SUBSCRIPTION: usize = LEN_SUBSCRIPTION_INFO + LEN_CHANNEL_SECRET;
pub const LEN_ENCRYPTED_SUBSCRIPTION: usize = LEN_STORED_SUBSCRIPTION + LEN_ASCON_AEAD_OVERHEAD;

// List subscription constants
pub const LEN_STANDARD_CHANNELS: usize = MAX_STANDARD_CHANNEL as usize;
pub const LEN_SUBSCRIPTION_INFO_LIST: usize = 4 + LEN_STANDARD_CHANNELS * LEN_SUBSCRIPTION_INFO; // The 4 accounts for the 32-bit "number of channels" requirement in host tools

// Frame and picture constants
pub const LEN_PICTURE_LEN: usize = 1;
pub const MAX_LEN_PICTURE: usize = 64;
pub const LEN_ENCRYPTED_PICTURE: usize = MAX_LEN_PICTURE + LEN_ASCON_AEAD_OVERHEAD;
pub const LEN_DECRYPTED_FRAME: usize =
    LEN_ENCRYPTED_PICTURE + LEN_CHANNEL_ID + LEN_TIMESTAMP + LEN_PICTURE_LEN;
pub const LEN_ENCRYPTED_FRAME: usize = LEN_DECRYPTED_FRAME + LEN_ASCON_AEAD_OVERHEAD;
