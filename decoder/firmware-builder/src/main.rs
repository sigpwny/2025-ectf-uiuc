use common::DeploymentSecrets;
use rand::Rng;
use std::fs::File;
use std::io::{Read, Write};

pub const FIRMWARE_SIZE: usize = 0x38000;
pub const FLASH_PAGE_SIZE: usize = 0x2000;
pub const OUT_FIRMWARE: &str = "./out/max78000.bin";

// Define memory regions and sizes
struct Segment<'a> {
    path: &'a str,
    address: usize,
    max_size: usize,
}

fn main() -> std::io::Result<()> {
    let segments = [
        Segment {
            path: "../max78000/out/decoder.bin",
            address: 0x00000000,
            max_size: 26 * FLASH_PAGE_SIZE,
        },

        // page 27 (index 26) is the global secrets
        // page 28 (index 27) is the subscription

        // Segment {
        //     path: "",
        //     address: 26 * FLASH_PAGE_SIZE,
        //     max_size: 1 * FLASH_PAGE_SIZE,
        // }
    ];

    // Read in global.secrets
    let mut secrets_file = File::open("../max78000/global.secrets")?;
    let mut secrets_data = Vec::new();
    secrets_file.read_to_end(&mut secrets_data)?;
    let secrets: DeploymentSecrets = serde_json::from_slice(&secrets_data).unwrap();

    println!("Secrets: {:?}", secrets);

    // Fill firmware with random data
    let mut firmware = vec![0xFF; FIRMWARE_SIZE];
    rand::rng().fill(&mut firmware[..]);

    for segment in &segments {
        let mut file = File::open(segment.path)?;
        let mut data = Vec::new();
        file.read_to_end(&mut data)?;

        if data.len() > segment.max_size {
            panic!("{} exceeds max allowed size of {:#X} bytes!", segment.path, segment.max_size);
        }

        // Copy data to firmware
        let start = segment.address;
        let end = start + data.len();
        firmware[start..end].copy_from_slice(&data);
    }

    // Write to final firmware file
    let mut output = File::create(OUT_FIRMWARE)?;
    output.write_all(&firmware)?;

    println!("Firmware built successfully!");
    Ok(())
}
