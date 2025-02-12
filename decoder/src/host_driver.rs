use embedded_hal_nb::serial::{self, Error};
use core::convert::Infallible;
use core::marker::PhantomData;

/// The type of message being sent or received over the host transport interface.
// TODO: Set MessageType based on opcode
#[derive(Eq, PartialEq)]
pub enum MessageType {
    Decode,
    Subscribe,
    List,
    Ack,
    Error,
    Debug,
    Invalid
}

pub struct MessageHeader {
    magic: u8,
    opcode: MessageType,
    length: u16
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

    pub fn read_packet(&mut self, packet: &mut [u8], header: &mut MessageHeader) -> Result<(), SerialError> {

        // let mut header: MessageHeader = MessageHeader{magic: 0,opcode: 0,length: 0};

        match self.read_header(header) {
            Ok(_) => {},
            Err(e) => match e {
                embedded_hal_nb::nb::Error::Other(o) => return core::prelude::v1::Err(o),
                embedded_hal_nb::nb::Error::WouldBlock => {}
            },
        }
        
        /* We properly read in the header, based on this let's read some amount of bytes */
        
        if (header.opcode != MessageType::Ack) {
            self.write_packet(MessageType::Ack, 0, &[])?; // Need to write ack, dont know what that is lol
            if (header.length != 0) {
                /* Read bytes into buffer (for loop) */
            }
            if (header.length != 0) {
                if (self.write_packet(MessageType::Ack, 0, &[]).is_err()) {
                    /* propagate error */
                    
                }
            }
        }

        /* Skipping cmd part, come back to later */
        Ok(())

        

        



    }

    fn read_header(&mut self, header: &mut MessageHeader) -> Result<(), embedded_hal_nb::nb::Error<SerialError>> {
        let mut magic_value: u8 = self.uart.read()?;

        while magic_value != b'%' {
            magic_value = self.uart.read()?;
        }

        header.magic = magic_value;
        header.opcode = match self.uart.read()? {
            b'D' => MessageType::Decode,
            b'S' => MessageType::Subscribe,
            b'L' => MessageType::List,
            b'A' => MessageType::Ack,
            b'E' => MessageType::Error,
            b'G' => MessageType::Debug,
            _ => MessageType::Error
        };
        if (header.opcode == MessageType::Error) {
            header.length = 0;
            return Ok(());
        }
        let first_byte = self.uart.read()?; /* Ask if there is way to read multiple btes at a time, serial implement one byte while mebedded io does multiple */
        let second_byte = self.uart.read()?;
        header.length = ((((first_byte as u16) << 8) & 0xFF00) | (second_byte as u16 & 0x00FF));

        Ok(())

    }

    pub fn write_packet(&mut self, message_type: MessageType, len: u16, packet: &[u8]) -> Result<(), SerialError> {
        // for byte in packet {
        //     nb::block!(self.uart.write(*byte))?;
        // }



        

        Ok(())
    }
}