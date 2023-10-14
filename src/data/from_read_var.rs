use super::FromRead;

use duplicate::duplicate_item;
use std::error::Error as StdError;
use std::io::{Error as IoError, Read};

/// Provides a constructor that reads an [unsigned LEB128](https://en.wikipedia.org/wiki/LEB128#Unsigned_LEB128)
/// sequence from some source.
pub trait FromReadVar<T>: Sized {
    /// Error type on read failure.
    type Error: StdError;

    /// Constructs `T` by reading an [unsigned LEB128](https://en.wikipedia.org/wiki/LEB128#Unsigned_LEB128)
    /// sequence from the given source.
    ///
    /// pxtone only uses this encoding for 32-bit data types, so up to 5 bytes of encoded data will
    /// be consumed. Only the lower 4 bits of a potential 5th byte are used in the result.
    fn from_read_var<R: Read>(source: &mut R) -> Result<T, Self::Error>;
}

#[duplicate_item(
    _Num32_;
    [f32];
    [i32];
    [u32];
)]
impl FromReadVar<Self> for _Num32_ {
    type Error = IoError;

    fn from_read_var<R: Read>(source: &mut R) -> Result<Self, Self::Error> {
        // Adapted from <https://en.wikipedia.org/wiki/LEB128#Decode_unsigned_integer>...
        let mut result = 0_u32;
        for i in 0..5 {
            let byte = u8::from_read(source)?;
            result |= ((byte as u32) & 0b0111_1111) << (7 * i);
            if (byte & 0b1000_0000) == 0 {
                break;
            }
        }

        Ok(result.to_le_bytes()).map(Self::from_le_bytes)
    }
}
