use embedded_hal_nb::nb::block;
use embedded_hal_nb::serial::{self, Error};
use core::convert::Infallible;
use core::marker::PhantomData;

const PACKET_SIZE_LIMIT: u16 = 1000;

/// The type of message being sent or received over the host transport interface.
// TODO: Set MessageType based on opcode
#[derive(Eq, PartialEq, Clone)]
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
                _ => {}
            },
        }
        
        /* We properly read in the header, based on this let's read some amount of bytes */
        
        if header.opcode != MessageType::Ack {
            self.write_packet(MessageType::Ack, 0, &[])?; // Need to write ack, dont know what that is lol
            if header.length != 0 {
                /* Read bytes into buffer (for loop) */
                for val in packet.iter_mut() {
                    *val = block!(self.uart.read())?;
                }
            }
            if header.length != 0 {
                self.write_packet(MessageType::Ack, 0, &[])?;
            }
        }

        /* Skipping cmd part, come back to later */
        Ok(())

        

        



    }

    fn read_header(&mut self, header: &mut MessageHeader) -> Result<(), embedded_hal_nb::nb::Error<SerialError>> {
        let mut magic_value: u8 = block!(self.uart.read())?;

        while magic_value != b'%' {
            magic_value = block!(self.uart.read())?;
        }

        header.magic = magic_value;
        header.opcode = match block!(self.uart.read())? {
            b'D' => MessageType::Decode,
            b'S' => MessageType::Subscribe,
            b'L' => MessageType::List,
            b'A' => MessageType::Ack,
            b'E' => MessageType::Error,
            b'G' => MessageType::Debug,
            _ => MessageType::Error
        };
        if header.opcode == MessageType::Error {
            header.length = 0;
            return Ok(());
        }
        let first_byte = block!(self.uart.read())?; /* Ask if there is way to read multiple btes at a time, serial implement one byte while mebedded io does multiple */
        let second_byte = block!(self.uart.read())?;
        header.length = (((first_byte as u16) << 8) & 0xFF00) | (second_byte as u16 & 0x00FF);

        if header.length > PACKET_SIZE_LIMIT {
            header.opcode = MessageType::Error;
            header.length = 0;
        }

        Ok(())

    }

    fn write_header(&mut self, header: &MessageHeader) -> Result<(), embedded_hal_nb::nb::Error<SerialError>> {
        block!(self.uart.write(header.magic))?;
        match header.opcode {
            MessageType::Decode => block!(self.uart.write(b'D'))?,
            MessageType::Subscribe => block!(self.uart.write(b'S'))?,
            MessageType::List => block!(self.uart.write(b'L'))?,
            MessageType::Ack => block!(self.uart.write(b'A'))?,
            MessageType::Error => block!(self.uart.write(b'E'))?,
            MessageType::Debug => block!(self.uart.write(b'G'))?,
            _ => block!(self.uart.write(b'E'))?
        };
        
        block!(self.uart.write((header.length >> 8) as u8))?;
        block!(self.uart.write((header.length & 0x00FF) as u8))?;

        Ok(())

    }

    pub fn write_packet(&mut self, message_type: MessageType, len: u16, packet: &[u8]) -> Result<(), SerialError> {

        let header: MessageHeader = MessageHeader{magic: b'%', opcode: message_type.clone(), length: len};

        match self.write_header(&header) {
            Ok(_) => {},
            Err(e) => match e {
                embedded_hal_nb::nb::Error::Other(o) => return core::prelude::v1::Err(o),
                _ => {}
            },
        }

        if message_type == MessageType::Ack {
            return Ok(());
        }
        if message_type != MessageType::Debug {
            let mut ack_header: MessageHeader = MessageHeader{magic: b'%', opcode: MessageType::Error, length: 0};
            match self.read_header(&mut ack_header) {
                Ok(_) => {},
                Err(e) => match e {
                    embedded_hal_nb::nb::Error::Other(o) => return core::prelude::v1::Err(o),
                    _ => {}
                },
            }
        }

        if len > 0 && len < PACKET_SIZE_LIMIT {
            for val in packet.iter() {
                block!(self.uart.write(*val))?;
            }
            if message_type != MessageType::Debug {
                let mut ack_header: MessageHeader = MessageHeader{magic: b'%', opcode: MessageType::Error, length: 0};
                match self.read_header(&mut ack_header) {
                    Ok(_) => {},
                    Err(e) => match e {
                        embedded_hal_nb::nb::Error::Other(o) => return core::prelude::v1::Err(o),
                        _ => {}
                    },
                }
            }
        }


        Ok(())
    }
}