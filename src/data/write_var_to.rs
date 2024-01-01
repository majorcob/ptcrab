use super::WriteTo;

use duplicate::duplicate_item;
use std::error::Error as StdError;
use std::io::{Error as IoError, Seek, Write};

//--------------------------------------------------------------------------------------------------

/// Allows encoding `self` as an [unsigned LEB128](https://en.wikipedia.org/wiki/LEB128#Unsigned_LEB128)
/// sequence written to some sink.
pub trait WriteVarTo {
    /// Error type on write failure.
    type Error: StdError;

    /// Encodes `self` as an [unsigned LEB128](https://en.wikipedia.org/wiki/LEB128#Unsigned_LEB128)
    /// sequence and writes it to the given sink.
    ///
    /// Returns the stream position before writing.
    fn write_var_to<W: Write + Seek>(&self, sink: &mut W) -> Result<u64, Self::Error>;
}

#[duplicate_item(
    _Num32_;
    [f32];
    [i32];
    [u32];
)]
impl WriteVarTo for _Num32_ {
    type Error = IoError;

    fn write_var_to<W: Write + Seek>(&self, sink: &mut W) -> Result<u64, Self::Error> {
        let start_pos = sink.stream_position()?;

        // Adapted from <https://en.wikipedia.org/wiki/LEB128#Encode_unsigned_integer>...
        let mut value = u32::from_le_bytes(self.to_le_bytes());
        for _ in 0..5 {
            let mut byte = (value & 0b0111_1111) as u8;
            value >>= 7;
            if value != 0 {
                byte |= 0b1000_0000;
            }
            byte.write_to(sink)?;
            if value == 0 {
                break;
            }
        }

        Ok(start_pos)
    }
}

impl<X, Y> WriteVarTo for (X, Y)
where
    X: WriteVarTo<Error = IoError>,
    Y: WriteVarTo<Error = IoError>,
{
    type Error = IoError;

    fn write_var_to<W: Write + Seek>(&self, sink: &mut W) -> Result<u64, Self::Error> {
        self.0
            .write_var_to(sink)
            .and_then(|start_pos| self.1.write_var_to(sink).map(|_| start_pos))
    }
}
