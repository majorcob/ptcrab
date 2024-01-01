use duplicate::duplicate_item;
use std::error::Error as StdError;
use std::io::{Error as IoError, Seek, Write};

//--------------------------------------------------------------------------------------------------

/// Allows encoding `self` as bytes written to some sink.
pub trait WriteTo {
    /// Error type on write/seek failure.
    type Error: StdError;

    /// Encodes `self` as bytes and writes them to the given sink.
    ///
    /// Returns the stream position before writing.
    fn write_to<W: Write + Seek>(&self, sink: &mut W) -> Result<u64, Self::Error>;
}

impl<const N: usize> WriteTo for [u8; N] {
    type Error = IoError;

    fn write_to<W: Write + Seek>(&self, sink: &mut W) -> Result<u64, Self::Error> {
        sink.stream_position()
            .and_then(|start_pos| sink.write_all(self).map(|_| start_pos))
    }
}

#[duplicate_item(
    _Num_;
    [f32];
    [i8];
    [i16];
    [i32];
    [u8];
    [u16];
    [u32];
)]
impl WriteTo for _Num_ {
    type Error = IoError;

    fn write_to<W: Write + Seek>(&self, sink: &mut W) -> Result<u64, Self::Error> {
        self.to_le_bytes().write_to(sink)
    }
}

impl<X, Y> WriteTo for (X, Y)
where
    X: WriteTo<Error = IoError>,
    Y: WriteTo<Error = IoError>,
{
    type Error = IoError;

    fn write_to<W: Write + Seek>(&self, sink: &mut W) -> Result<u64, Self::Error> {
        self.0
            .write_to(sink)
            .and_then(|start_pos| self.1.write_to(sink).map(|_| start_pos))
    }
}
