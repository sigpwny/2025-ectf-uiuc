use bincode::{
    de::{Decode, read::Reader},
    enc::{Encoder, write::Writer},
    error::DecodeError,
    decode_from_reader,
};
use common::{MessageToDecoder, MessageFromDecoder, BINCODE_CONFIG};
use common::constants::*;
use core::convert::Infallible;
use embedded_hal_nb::nb::block;
use embedded_hal_nb::serial;

pub const MAX_MESSAGE_SIZE: usize = 0x400; // 1024 bytes
pub const BLOCK_SIZE: usize = 0x100; // 256 bytes

/// The type of message being sent or received over the host transport interface.
// TODO: Set MessageType based on opcode
#[derive(Eq, PartialEq, Clone)]
pub enum MessageType {
    Invalid,
    Decode,
    Subscribe,
    List,
    Ack,
    Error,
    Debug,
}

pub enum UartError {
    Decode(DecodeError),
    InvalidOpcode,
    InvalidLength,
}

pub enum UartState {
    None,
    NumBytesRead(usize)
}

pub struct MessageHeader {
    pub opcode: MessageType,
    pub length: u16,
}

impl MessageHeader {
    pub fn new() -> Self {
        Self {
            opcode: MessageType::Invalid,
            length: 0,
        }
    }

    /// Returns true if the message should be acknowledged.
    /// Should not ACK messages with opcode Ack or Debug.
    pub fn should_ack(&self) -> bool {
        match self.opcode {
            MessageType::Ack | MessageType::Debug => false,
            _ => true,
        }
    }
}

pub struct Message {
    pub header: MessageHeader,
    pub data: [u8; MAX_MESSAGE_SIZE],
}

impl Message {
    pub fn new() -> Self {
        Self {
            header: MessageHeader {
                opcode: MessageType::Invalid,
                length: 0,
            },
            data: [0u8; MAX_MESSAGE_SIZE],
        }
    }

    pub fn error() -> Self {
        Self {
            header: MessageHeader {
                opcode: MessageType::Error,
                length: 0,
            },
            data: [0u8; MAX_MESSAGE_SIZE],
        }
    }

    pub fn ack() -> Self {
        Self {
            header: MessageHeader {
                opcode: MessageType::Ack,
                length: 0,
            },
            data: [0u8; MAX_MESSAGE_SIZE],
        }
    }

    pub fn debug(message: &[u8]) -> Self {
        let mut data = [0u8; MAX_MESSAGE_SIZE];
        data[..message.len()].copy_from_slice(message);
        Self {
            header: MessageHeader {
                opcode: MessageType::Debug,
                length: message.len() as u16,
            },
            data,
        }
    }

    pub fn decode(message: &[u8]) -> Self {
        let mut data = [0u8; MAX_MESSAGE_SIZE];
        data[..message.len()].copy_from_slice(message);
        Self {
            header: MessageHeader {
                opcode: MessageType::Decode,
                length: message.len() as u16,
            },
            data,
        }
    }
}

/// A driver for the host computer and decoder interface as described in the
/// [eCTF 2025 Detailed Specifications](https://rules.ectf.mitre.org/2025/specs/detailed_specs.html).
pub struct HostDriver<Serial, SerialError = Infallible>
where
    Serial: serial::Read<u8, Error = SerialError> + serial::Write<u8, Error = SerialError>,
{
    uart: Serial,
    state: UartState,
}

impl<Serial, SerialError> Reader for HostDriver<Serial, SerialError>
where
    Serial: serial::Read<u8, Error = SerialError> + serial::Write<u8, Error = SerialError>,
{
    fn read(&mut self, buf: &mut [u8]) -> Result<(), DecodeError> {
        for b in buf.iter_mut() {
            self.state = match self.state {
                UartState::NumBytesRead(0) => {
                    self.write_ack();
                    UartState::NumBytesRead(1)
                }
                UartState::NumBytesRead(n @ 0..255) => UartState::NumBytesRead(n + 1),
                UartState::NumBytesRead(255) => UartState::NumBytesRead(0),
                _ => unreachable!("Invalid state"),
            };
            *b = block!(self.uart.read()).map_err(|_| DecodeError::Other("UART read error"))?
        }
        Ok(())
    }
}

impl<Serial, SerialError> HostDriver<Serial, SerialError>
where
    Serial: serial::Read<u8, Error = SerialError> + serial::Write<u8, Error = SerialError>,
{
    /// Create a new host transport driver with the given serial interface.
    pub fn new(uart: Serial) -> Self {
        Self { uart, state: UartState::None }
    }

    /// Free the serial interface from the host transport driver.
    pub fn free(self) -> Serial {
        self.uart
    }

    /// Read a message from the host computer.
    pub fn read_message(&mut self) -> Result<MessageToDecoder, UartError> {
        let header = self.read_header()?;

        self.state = UartState::NumBytesRead(0);

        let result = match (header.opcode, header.length as usize) {
            (MessageType::List, 0) => Ok(MessageToDecoder::ListSubscriptions),
            (MessageType::Subscribe, LEN_ENCRYPTED_SUBSCRIPTION) => Ok(MessageToDecoder::UpdateSubscription(
                decode_from_reader(&mut *self, BINCODE_CONFIG).map_err(|e| UartError::Decode(e))?,
            )),
            (MessageType::Decode, LEN_ENCRYPTED_FRAME) => Ok(MessageToDecoder::DecodeFrame(decode_from_reader(&mut *self, BINCODE_CONFIG).map_err(|e| UartError::Decode(e))?)),
            (MessageType::List|MessageType::Subscribe|MessageType::Decode, _) => Err(UartError::InvalidLength),
            _ => Err(UartError::InvalidOpcode),
        };

        self.write_ack();

        result

        // let mut message = Message::new();

        // TODO: Add random delay here

        // Keep reading headers until we get a valid one
        // loop {
        //     let res = self.read_header();
        //     self.write_ack();
        //     match res {
        //         Ok(header) => {
        //             message.header = header;
        //             break;
        //         }
        //         Err(_) => {
        //             self.write_message(Message::error());
        //             self.read_ack();
        //         }
        //     }
        // }

        // // Read the message data in blocks of BLOCK_SIZE
        // // After each block, send an ACK message
        // // Read until either message.data is full or the header length is reached
        // let mut bytes_read = 0;
        // let read_limit = core::cmp::min(message.header.length as usize, message.data.len());
        // while bytes_read < read_limit {
        //     let end = core::cmp::min(bytes_read + BLOCK_SIZE, read_limit);
        //     for i in bytes_read..end {
        //         if let Ok(val) = block!(self.uart.read()) {
        //             message.data[i] = val;
        //             bytes_read += 1;
        //         }
        //     }
        //     self.write_ack();
        // }

        // // TODO: Add random delay here

        // message
    }

    /// Write a message to the host computer.
    pub fn write_message(&mut self, message: Message) {
        // TODO: Add random delay here
        let _ = self.write_header(&message.header);
        if message.header.should_ack() {
            self.read_ack();
        }
        // Read data in blocks of BLOCK_SIZE (if necessary)
        let mut bytes_written = 0;
        let write_limit = core::cmp::min(message.header.length as usize, message.data.len());
        while bytes_written < write_limit {
            let end = core::cmp::min(bytes_written + BLOCK_SIZE, write_limit);
            for i in bytes_written..end {
                if let Ok(_) = block!(self.uart.write(message.data[i])) {
                    bytes_written += 1;
                }
            }
            if message.header.should_ack() {
                self.read_ack();
            }
        }
        // TODO: Add random delay here
    }

    /// Read an ACK message from the host computer. Blocks until an ACK is received.
    pub fn read_ack(&mut self) {
        let mut ack_header = MessageHeader::new();
        while ack_header.opcode != MessageType::Ack {
            match self.read_header() {
                Ok(header) => ack_header = header,
                Err(_) => continue,
            }
        }
    }

    /// Write an ACK message to the host computer.
    pub fn write_ack(&mut self) {
        self.write_message(Message::ack());
    }

    /// Helper function to read a header from the host computer.
    fn read_header(&mut self) -> Result<MessageHeader, UartError> {
        let mut header = MessageHeader::new();
        let mut magic_value = 0u8;

        // Magic
        while magic_value != b'%' {
            match block!(self.uart.read()) {
                Ok(val) => magic_value = val,
                Err(_) => return Err(UartError::Decode(DecodeError::Other("UART read error"))),
            }
        }

        // Opcode
        match block!(self.uart.read()) {
            Ok(val) => {
                header.opcode = match val {
                    b'D' => MessageType::Decode,
                    b'S' => MessageType::Subscribe,
                    b'L' => MessageType::List,
                    b'A' => MessageType::Ack,
                    b'E' => MessageType::Error,
                    b'G' => MessageType::Debug,
                    _ => return Err(UartError::InvalidOpcode),
                }
            }
            Err(_) => return Err(UartError::Decode(DecodeError::Other("UART read error"))),
        };

        // Length
        let mut length_bytes = [0u8; 2];
        for b in length_bytes.iter_mut() {
            match block!(self.uart.read()) {
                Ok(val) => *b = val,
                Err(_) => return Err(UartError::Decode(DecodeError::Other("UART read error"))),
            }
        }
        header.length = u16::from_le_bytes(length_bytes);

        Ok(header)
    }

    /// Helper function to serialize a header for the host computer.
    fn write_header(
        &mut self,
        header: &MessageHeader,
    ) -> Result<(), embedded_hal_nb::nb::Error<SerialError>> {
        let opcode = match header.opcode {
            MessageType::Decode => b'D',
            MessageType::Subscribe => b'S',
            MessageType::List => b'L',
            MessageType::Ack => b'A',
            MessageType::Error => b'E',
            MessageType::Debug => b'G',
            _ => b'E',
            // _ => return Err(embedded_hal_nb::nb::Error::Other(ErrorHeader::InvalidOpcode))
        };

        let length_bytes = header.length.to_le_bytes();

        block!(self.uart.write(b'%'))?;
        block!(self.uart.write(opcode))?;
        for b in length_bytes.iter() {
            block!(self.uart.write(*b))?;
        }

        Ok(())
    }
}
