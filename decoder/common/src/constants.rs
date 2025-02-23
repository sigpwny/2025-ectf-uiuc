// Number-related constants
pub const MAX_STANDARD_CHANNEL: u32 = 8;

// Encryption-related constants
pub const AEAD_ENCRYPTION_OVERHEAD: usize = 16;
pub const LEN_ASCON_KEY: usize = 16;
pub const LEN_BASE_CHANNEL_SECRET: usize = 32;
pub const LEN_BASE_SUBSCRIPTION_SECRET: usize = 32;
pub const LEN_CHANNEL_SECRET: usize = 32;

// Subscription-related constants
pub const LEN_CHANNEL_ID: usize = 4;
pub const LEN_TIMESTAMP: usize = 8;
pub const LEN_SUBSCRIPTION_INFO: usize = LEN_CHANNEL_ID + 2 * LEN_TIMESTAMP;
pub const LEN_STORED_SUBSCRIPTION: usize = LEN_SUBSCRIPTION_INFO + LEN_CHANNEL_SECRET;
pub const LEN_ENCRYPTED_SUBSCRIPTION: usize = LEN_SUBSCRIPTION_INFO + AEAD_ENCRYPTION_OVERHEAD;

// Subscription list-related constants
pub const LEN_STANDARD_CHANNELS: usize = MAX_STANDARD_CHANNEL as usize;
pub const LEN_SUBSCRIPTION_INFO_LIST: usize = LEN_STANDARD_CHANNELS * LEN_SUBSCRIPTION_INFO;
pub const LEN_SUBSCRIPTION_INFO_LIST_BYTES: usize = LEN_SUBSCRIPTION_INFO_LIST + 4;  // The 4 accounts for the 32-bit "number of channels" requirement in host tools
pub const LEN_STORED_SUBSCRIPTION_LIST: usize = LEN_STANDARD_CHANNELS * LEN_STORED_SUBSCRIPTION;
pub const LEN_STORED_SUBSCRIPTION_LIST_BYTES: usize = LEN_STORED_SUBSCRIPTION_LIST + 1;

// Frame-related constants
pub const LEN_FRAME_LENGTH: usize = 1;
pub const LEN_PICTURE: usize = 64;
pub const LEN_ENCRYPTED_PICTURE: usize = LEN_PICTURE + AEAD_ENCRYPTION_OVERHEAD;
pub const LEN_DECRYPTED_FRAME: usize = LEN_ENCRYPTED_PICTURE + LEN_CHANNEL_ID + LEN_TIMESTAMP + LEN_FRAME_LENGTH;
pub const LEN_ENCRYPTED_FRAME: usize = LEN_DECRYPTED_FRAME + AEAD_ENCRYPTION_OVERHEAD;
