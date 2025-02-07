#![no_std]

use serde::{ser, Serialize};
use crate::Error;

pub struct Serializer {
    output: &'static mut [u8]
}


pub fn to_bytes<T>(value: &T, output: &'static mut [u8]) -> Result<&'static mut [u8], Error>
where
    T: Serialize,
{
    let mut serializer = Serializer { output };
    value.serialize(&mut serializer)?;
    Ok(serializer.output)
}

impl<'a> ser::Serializer for &'a mut Serializer {
    unimplemented!();
}
