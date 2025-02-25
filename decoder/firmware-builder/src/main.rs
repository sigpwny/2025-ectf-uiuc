use clap::Parser;
use common::{DeploymentSecrets, SubscriptionInfo, StoredSubscription};
use common::constants::*;
use common::crypto::{derive_subscription_key, derive_channel_secret};
use rand::Rng;
use std::fs::File;
use std::io::{Read, Write};

pub const DEFAULT_OUT_FIRMWARE: &str = "./out/max78000.bin";

/// Parses a number that can be either decimal (e.g., `42`) or hexadecimal (e.g., `0x2A`)
fn parse_number(s: &str) -> Result<u32, String> {
    if let Some(hex) = s.strip_prefix("0x") {
        u32::from_str_radix(hex, 16).map_err(|_| format!("Invalid hexadecimal number: {}", s))
    } else {
        s.parse::<u32>().map_err(|_| format!("Invalid decimal number: {}", s))
    }
}

#[derive(Debug, Parser)]
struct Args {
    /// Path to the firmware binary
    #[arg(short, long, value_name = "FILE")]
    firmware: String,

    /// Path to the global secrets file
    #[arg(short, long, value_name = "FILE")]
    secrets: String,

    /// Decoder ID
    #[arg(short, long, value_parser = parse_number)]
    decoder_id: u32,

    /// Path to the output firmware binary
    #[arg(short, long, value_name = "FILE", default_value = DEFAULT_OUT_FIRMWARE)]
    output: String,
}

// Define memory regions and sizes
struct Segment<'a> {
    path: &'a str,
    address: usize,
    max_size: usize,
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();

    println!("Building firmware for DECODER_ID: {:#010x}", args.decoder_id);

    // Read in global.secrets
    let mut secrets_file = File::open(args.secrets)?;
    let mut secrets_data = Vec::new();
    secrets_file.read_to_end(&mut secrets_data)?;
    let secrets: DeploymentSecrets = serde_json::from_slice(&secrets_data).expect("Failed to deserialize deployment secrets");

    // Fill firmware with random data
    let mut output_firmware = vec![0xFF; FLASH_FIRMWARE_SIZE as usize];
    rand::rng().fill(&mut output_firmware[..]);

    // Read and verify input firmware binary
    let input_firmware = Segment {
        path: args.firmware.as_str(),
        address: 0x00000000,
        max_size: FLASH_FIRMWARE_CODE_SIZE as usize,
    };
    let mut file = File::open(input_firmware.path)?;
    let mut data = Vec::new();
    file.read_to_end(&mut data)?;
    if data.len() > input_firmware.max_size {
        panic!("{} exceeds max allowed size of {:#X} bytes!", input_firmware.path, input_firmware.max_size);
    }
    // Copy data to firmware
    let start = input_firmware.address;
    let end = start + data.len();
    output_firmware[start..end].copy_from_slice(&data);

    // Write frame key to firmware
    let frame_key_start = FLASH_OFFSET_FRAME_KEY as usize;
    let frame_key_end = frame_key_start + LEN_ASCON_KEY;
    output_firmware[frame_key_start..frame_key_end].copy_from_slice(&secrets.frame_key.0);
    // Derive subscription key from secrets
    let subscription_key = derive_subscription_key(&secrets.base_subscription_secret, args.decoder_id);
    // Write subscription key to firmware
    let subscription_key_start = FLASH_OFFSET_SUBSCRIPTION_KEY as usize;
    let subscription_key_end = subscription_key_start + LEN_ASCON_KEY;
    output_firmware[subscription_key_start..subscription_key_end].copy_from_slice(&subscription_key.0);

    // Write channel 0 subscription to firmware
    let channel_0_subscription_start = FLASH_OFFSET_SUBSCRIPTION_BASE as usize;
    let channel_0_subscription_end = channel_0_subscription_start + LEN_STORED_SUBSCRIPTION;
    let channel_0_secret = derive_channel_secret(&secrets.base_channel_secret, 0);
    let channel_0_subscription = StoredSubscription {
        info: SubscriptionInfo {
            channel_id: 0,
            start: 0,
            end: u64::MAX,
        },
        channel_secret: channel_0_secret,
    };
    // TODO: Implement channel 0 subscription
    unimplemented!("Channel 0 subscription needs to be saved to firmware!");

    // Write to final firmware file
    let mut output = File::create(args.output)?;
    output.write_all(&output_firmware)?;

    println!("Firmware built successfully!");
    Ok(())
}
