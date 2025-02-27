#![no_std]
#![no_main]

pub mod crypto;
pub mod decode;
pub mod hardening;
pub mod host_driver;
pub mod rng;
pub mod subscription;
pub mod tmr;

pub extern crate max7800x_hal as hal;
pub use hal::entry;
pub use hal::pac;

// TODO: Custom panic handler
use panic_halt as _;

use common::constants::*;
use common::{MessageToDecoder, Timestamp};
use decode::{decrypt_frame, validate_and_decrypt_picture};
use host_driver::{HostDriver, Message};
use rng::new_custom_rng;
use subscription::{decrypt_subscription, list_subscriptions, update_subscription};
use tmr::Tmr2;

#[entry]
fn main() -> ! {
    let p = pac::Peripherals::take().expect("Failed to take peripherals");
    let core = pac::CorePeripherals::take().expect("Failed to take core peripherals");

    let mut gcr = hal::gcr::Gcr::new(p.gcr, p.lpgcr);
    let ipo = hal::gcr::clocks::Ipo::new(gcr.osc_guards.ipo).enable(&mut gcr.reg);
    let clks = gcr
        .sys_clk
        .set_source(&mut gcr.reg, &ipo)
        .set_divider::<hal::gcr::clocks::Div1>(&mut gcr.reg)
        .freeze();

    // Initialize a delay timer using the ARM SYST (SysTick) peripheral
    let rate = clks.sys_clk.frequency;
    let host_delay = cortex_m::delay::Delay::new(core.SYST, rate);

    // Initialize and split the GPIO0 peripheral into pins
    let gpio0_pins = hal::gpio::Gpio0::new(p.gpio0, &mut gcr.reg).split();
    // Configure UART to host computer with 115200 8N1 settings
    let rx_pin = gpio0_pins.p0_0.into_af1();
    let tx_pin = gpio0_pins.p0_1.into_af1();
    let host_uart = hal::uart::UartPeripheral::uart0(p.uart0, &mut gcr.reg, rx_pin, tx_pin)
        .baud(115200)
        .clock_pclk(&clks.pclk)
        .parity(hal::uart::ParityBit::None)
        .build();

    // Initialize TRNG peripheral
    let trng = hal::trng::Trng::new(p.trng, &mut gcr.reg);

    // Initialize TMR2 peripheral
    let tmr2 = Tmr2::new(p.tmr2, &mut gcr.reg);
    tmr2.config();

    // Initialize the FLC peripheral
    let mut flc = hal::flc::Flc::new(p.flc, clks.sys_clk);

    // Initialize the custom RNG
    let rng_seed =
        unsafe { core::ptr::read_volatile(FLASH_ADDR_RANDOM_BYTES as *const [u8; LEN_RNG_SEED]) };
    let host_rng = new_custom_rng(&rng_seed, &trng, &tmr2);

    // Initialize the monotonic timestamp tracker
    let mut timestamp = Timestamp(0);

    // Iniitialize the host transport driver
    let mut host = HostDriver::new(host_uart, host_rng, host_delay);

    loop {
        let message = host.read_message();
        match message {
            Ok(MessageToDecoder::ListSubscriptions) => {
                let sub_list = list_subscriptions(&mut flc);
                assert!(sub_list.num_sub_channels <= LEN_STANDARD_CHANNELS as u32);
                let mut m = Message::list();
                m.add_data(&sub_list.num_sub_channels.to_le_bytes());
                for i in 0..sub_list.num_sub_channels {
                    m.add_data(&sub_list.subscriptions[i as usize].channel_id.to_le_bytes());
                    m.add_data(&sub_list.subscriptions[i as usize].start.to_le_bytes());
                    m.add_data(&sub_list.subscriptions[i as usize].end.to_le_bytes());
                }
                host.write_message(m);
            }
            Ok(MessageToDecoder::UpdateSubscription(enc_subscription)) => {
                match decrypt_subscription(enc_subscription) {
                    Ok(new_sub) => match update_subscription(&mut flc, new_sub) {
                        Ok(_) => host.write_message(Message::subscribe()),
                        Err(_) => host.error(),
                    },
                    Err(_) => host.error(),
                }
            }
            Ok(MessageToDecoder::DecodeFrame(enc_frame)) => {
                match decrypt_frame(&enc_frame) {
                    Ok(dec_frame) => {
                        // TODO: Add random delay here
                        match validate_and_decrypt_picture(&mut flc, &mut timestamp, &dec_frame) {
                            Ok(pic) => {
                                let mut m = Message::decode();
                                m.add_data_bounded(&pic.picture.0, pic.picture_length as usize);
                                host.write_message(m);
                            }
                            Err(_) => host.error(),
                        }
                    }
                    Err(_) => host.error(),
                }
            }
            Err(_) => host.error(),
        };
    }
}
