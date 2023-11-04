//! pxtone voice (ptvoice) functionality.
//!
//! Not to be confused with a "project voice", which includes ptvoices, ptnoises, and PCM/Vorbis
//! samples.

mod error;
mod unit;
mod wave;
pub use self::error::*;
pub use self::unit::*;
pub use self::wave::*;

use crate::data::{FromRead, FromReadVar};
use crate::Key;

use std::io::Read;

//--------------------------------------------------------------------------------------------------

type PtvSignature = [u8; 8];

/// Synthesized instrument made up of sine overtones and drawn waveforms.
pub struct Ptvoice {
    /// Basic-key applied to the entire project voice in old pxtone versions. Each voice-unit has
    /// its own basic-key in newer versions, so this is set to 0 and goes unused.
    pub legacy_basic_key: Key,
    /// ptvoices can contain multiple units, each with its own waveform, parameters, and envelope.
    pub units: Box<[PtvUnit]>,
}

impl Ptvoice {
    /// String present at the start of ptvoice data.
    const SIGNATURE: PtvSignature = *b"PTVOICE-";
    /// Maximum supported format version.
    #[allow(clippy::inconsistent_digit_grouping)]
    const VERSION: i32 = 2006_01_11;
}

impl FromRead<Self> for Ptvoice {
    type Error = PtvError;

    fn from_read<R: Read>(source: &mut R) -> Result<Self, Self::Error> {
        // Check signature at start of data.
        if Self::SIGNATURE != PtvSignature::from_read(source)? {
            return Err(PtvError::Invalid);
        }
        // Check that format version is supported.
        if Self::VERSION < i32::from_read(source)? {
            return Err(PtvError::Unsupported);
        }
        // Length of remaining data. pxtone doesn't actually verify this.
        let _data_len = i32::from_read(source)?;

        let legacy_basic_key = i32::from_read_var(source)?.into();
        // Reserved two zeroes.
        for _ in 0..2 {
            if i32::from_read_var(source)? != 0 {
                return Err(PtvError::Invalid)?;
            }
        }

        // Read units...
        let unit_count =
            usize::try_from(i32::from_read_var(source)?).map_err(|_| PtvError::Invalid)?;
        let units = (0..unit_count)
            .map(|_| PtvUnit::from_read(source))
            .collect::<Result<Box<[_]>, _>>()?;

        Ok(Self {
            legacy_basic_key,
            units,
        })
    }
}
