use clap::Parser;
use common::{
    DeploymentSecrets,
    make_complement_16b,
};
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

    // Set up channel 0 subscription
    let c0_id = EMERGENCY_CHANNEL_ID;
    let c0_secret = derive_channel_secret(&secrets.base_channel_secret, c0_id);
    let c0_start: u64 = 0;
    let c0_end: u64 = u64::MAX;

    let mut sub_bytes = [0u8; 128];

    // Write the header
    let mut header_bytes = [FLASH_MAGIC_SUBSCRIPTION; 16];
    header_bytes[4..8].copy_from_slice(&c0_id.to_le_bytes());
    header_bytes[12..16].copy_from_slice(&c0_id.to_le_bytes());
    sub_bytes[0..16].copy_from_slice(&header_bytes);
    sub_bytes[16..32].copy_from_slice(&make_complement_16b(&header_bytes));

    // Write the timestamps
    let mut timestamp_bytes = [0u8; 16];
    timestamp_bytes[0..8].copy_from_slice(&c0_start.to_le_bytes());
    timestamp_bytes[8..16].copy_from_slice(&c0_end.to_le_bytes());
    sub_bytes[32..48].copy_from_slice(&timestamp_bytes);
    sub_bytes[48..64].copy_from_slice(&make_complement_16b(&timestamp_bytes));

    // Write the channel secret
    let mut channel_secret_bytes_1 = [0u8; 16];
    channel_secret_bytes_1.copy_from_slice(&c0_secret.0[0..16]);
    sub_bytes[64..80].copy_from_slice(&channel_secret_bytes_1);
    sub_bytes[80..96].copy_from_slice(&make_complement_16b(&channel_secret_bytes_1));
    let mut channel_secret_bytes_2 = [0u8; 16];
    channel_secret_bytes_2.copy_from_slice(&c0_secret.0[16..32]);
    sub_bytes[96..112].copy_from_slice(&channel_secret_bytes_2);
    sub_bytes[112..128].copy_from_slice(&make_complement_16b(&channel_secret_bytes_2));

    // Write subscription to firmware
    let c0_sub_start = FLASH_OFFSET_SUBSCRIPTION_BASE as usize;
    output_firmware[c0_sub_start..c0_sub_start + 128].copy_from_slice(&sub_bytes);

    // Write to final firmware file
    let mut output = File::create(args.output)?;
    output.write_all(&output_firmware)?;

    println!("Firmware built successfully!");
    Ok(())
}
