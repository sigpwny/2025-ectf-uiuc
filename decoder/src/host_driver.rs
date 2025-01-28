use embedded_hal_nb::serial;
use core::convert::Infallible;
use core::marker::PhantomData;

/// The type of message being sent or received over the host transport interface.
// TODO: Set MessageType based on opcode
pub enum MessageType {
    Decode,
    Subscribe,
    List,
    Ack,
    Error,
    Debug,
}

/// A driver for the host computer and decoder interface as described in the
/// [eCTF 2025 Detailed Specifications](https://rules.ectf.mitre.org/2025/specs/detailed_specs.html).
pub struct HostTransportDriver<Serial, SerialError = Infallible>
where
    Serial: serial::Read<u8, Error = SerialError> + serial::Write<u8, Error = SerialError>,
{
    uart: Serial,
    _error: PhantomData<SerialError>,
}

impl<Serial, SerialError> HostTransportDriver<Serial, SerialError>
where 
    Serial: serial::Read<u8, Error = SerialError> + serial::Write<u8, Error = SerialError>,
{
    /// Create a new host transport driver with the given serial interface.
    pub fn new(uart: Serial) -> Self {
        Self {
            uart,
            _error: PhantomData,
        }
    }

    /// Free the serial interface from the host transport driver.
    pub fn free(self) -> Serial {
        self.uart
    }

    // pub fn read_packet(&mut self, packet: &mut [u8]) -> Result<(), SerialError> {
    //     for byte in packet.iter_mut() {
    //         *byte = nb::block!(self.uart.read())?;
    //     }
    //     Ok(())
    // }

    // pub fn write_packet(&mut self, packet: &[u8]) -> Result<(), SerialError> {
    //     for byte in packet {
    //         nb::block!(self.uart.write(*byte))?;
    //     }
    //     Ok(())
    // }
}