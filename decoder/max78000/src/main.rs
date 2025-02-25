#![no_std]
#![no_main]

pub mod crypto;
pub mod decode;
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
use common::MessageToDecoder;
use host_driver::{HostDriver, Message, MessageType};
use rng::new_custom_rng;
use tmr::Tmr2;
use rand::RngCore;

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
    let mut delay = cortex_m::delay::Delay::new(core.SYST, rate);

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

    // Initialize the custom RNG
    // TODO: Seed the RNG with a unique value
    let rng_seed = [0u8; LEN_RNG_SEED];
    let mut random = new_custom_rng(&rng_seed, &trng, &tmr2);
    // let mut random2 = new_custom_rng(&tmr2, &trng, [1u8; 64]);

    // Iniitialize the host transport driver
    let mut host = HostDriver::new(host_uart);

    // TODO: Remove debug loop
    // for _ in 0..3 {
    //     host_driver.write_message(Message::debug(b"Hello from the host driver!".as_slice()));
    //     delay.delay_ms(500);
    // }

    loop {
        let message = host.read_message();
        // let res: Result<MessageToDecoder, DecodeError> = bincode::decode_from_reader(&mut host, config);
        match message {
            Ok(MessageToDecoder::ListSubscriptions) => {
                unimplemented!("List message not implemented");
            },
            Ok(MessageToDecoder::UpdateSubscription(enc_subscription)) => {
                unimplemented!("Subscribe message not implemented");
            },
            Ok(MessageToDecoder::DecodeFrame(enc_frame)) => {
                // handle_decode(host, enc_frame);
                // unimplemented!("Decode message not implemented");
                let decoded_frame = b"Hello from the decoder!".as_slice();
                let mut decode_message = Message::new();
                decode_message.header.opcode = MessageType::Decode;
                decode_message.header.length = decoded_frame.len() as u16;
                decode_message.data[..decoded_frame.len()].copy_from_slice(decoded_frame);
                host.write_message(decode_message);
            },
            Err(_) => {
                host.write_message(Message::error());
            }
        };
        // match message.header.opcode {
        //     MessageType::List => {
        //         unimplemented!("List message not implemented");
        //     }
        //     MessageType::Subscribe => {
        //         unimplemented!("Subscribe message not implemented");
        //     }
        //     MessageType::Decode => handle_decode(host, message),
        //     _ => {}
        // }
    }
}

// fn handle_decode(host: &mut HostDriver, message: Message) {
//     if message.header.length != LEN_ENCRYPTED_FRAME {
//         host.write_message(Message::error());
//         return;
//     }
//     let enc_frame = EncryptedFrame::from_bytes(&message.data);
//     match decode::decode_frame(enc_frame) {
//         Ok(response) => {
//             host.write_message(Message::decode(&response));
//         }
//         Err(_) => {
//             host.write_message(Message::error());
//         }
//     }
// }