//! pxtone voice (ptvoice) functionality.
//!
//! Not to be confused with a "project voice", which includes ptvoices, ptnoises, and PCM/Vorbis
//! samples.

mod envelope;
mod error;
mod unit;
mod wave;
pub use self::envelope::*;
pub use self::error::*;
pub use self::unit::*;
pub use self::wave::*;

use crate::data::{FromRead, FromReadVar, WriteTo, WriteVarTo};

use std::io::SeekFrom;
use std::io::{Read, Seek, Write};

//--------------------------------------------------------------------------------------------------

type PtvSignature = [u8; 8];

/// Synthesized instrument made up of sine overtones and drawn waveforms.
#[derive(Clone, Debug, PartialEq)]
pub struct Ptvoice {
    /// Basic key applied to the entire project voice in old pxtone versions. Each voice-unit has
    /// its own basic key in newer versions, so this is set to 0 and goes unused.
    pub legacy_basic_key: i32,
    /// ptvoices can contain multiple units, each with its own waveform, parameters, and envelope.
    ///
    /// The official ptvoice editor will refuse to open a ptvoice containing >2 units, and pxtone
    /// Collage will usually crash when attempting to preview such a ptvoice. However, playback
    /// actually seems to work and will properly render all voice-units. Use with caution.
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

        let legacy_basic_key = i32::from_read_var(source)?;
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

impl WriteTo for Ptvoice {
    type Error = PtvError;

    fn write_to<W: Write + Seek>(&self, sink: &mut W) -> Result<u64, Self::Error> {
        // ptvoice signature and format version.
        let start_pos = Self::SIGNATURE.write_to(sink)?;
        Self::VERSION.write_to(sink)?;
        // Placeholder for remaining data length (to be written later).
        let data_len_pos = 0_i32.write_to(sink)?;

        // Legacy basic-key.
        let data_start = self.legacy_basic_key.write_var_to(sink)?;
        // Reserved data.
        0_i32.write_var_to(sink)?;
        0_i32.write_var_to(sink)?;

        // Units...
        i32::try_from(self.units.len())
            .map_err(|_| PtvError::OverMax)?
            .write_var_to(sink)?;
        for unit in self.units.iter() {
            unit.write_to(sink)?;
        }

        // Go back to update data length.
        let data_end = sink.stream_position()?;
        let data_len = i32::try_from(data_end - data_start).map_err(|_| PtvError::OverMax)?;
        sink.seek(SeekFrom::Start(data_len_pos))?;
        data_len.write_to(sink)?;
        sink.seek(SeekFrom::Start(data_end))?;

        Ok(start_pos)
    }
}

impl Default for Ptvoice {
    fn default() -> Self {
        Self {
            legacy_basic_key: 0,
            units: Box::new([PtvUnit::default()]),
        }
    }
}
