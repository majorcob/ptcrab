use duplicate::duplicate_item;
use std::error::Error as StdError;
use std::io::{Error as IoError, Read};

//--------------------------------------------------------------------------------------------------

/// Provides a constructor that reads data from some source.
pub trait FromRead<T>: Sized {
    /// Error type on read failure.
    type Error: StdError;

    /// Constructs `T` by reading data from a given source.
    fn from_read<R: Read>(source: &mut R) -> Result<T, Self::Error>;
}

impl<const N: usize> FromRead<Self> for [u8; N] {
    type Error = IoError;

    fn from_read<R: Read>(source: &mut R) -> Result<Self, Self::Error> {
        let mut buffer = [0_u8; N];

        source.read_exact(&mut buffer).map(|_| buffer)
    }
}

/// Returns an array of `N` bytes read from a source.
#[inline]
fn read_n<const N: usize, R: Read>(source: &mut R) -> Result<[u8; N], IoError> {
    <[u8; N]>::from_read(source)
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
impl FromRead<Self> for _Num_ {
    type Error = IoError;

    fn from_read<R: Read>(source: &mut R) -> Result<Self, Self::Error> {
        read_n(source).map(Self::from_le_bytes)
    }
}

impl<X, Y> FromRead<Self> for (X, Y)
where
    X: FromRead<X, Error = IoError>,
    Y: FromRead<Y, Error = IoError>,
{
    type Error = IoError;

    fn from_read<R: Read>(source: &mut R) -> Result<Self, Self::Error> {
        X::from_read(source).and_then(|x| Y::from_read(source).map(|y| (x, y)))
    }
}
