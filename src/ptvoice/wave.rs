use super::PtvError;
use crate::data::{FromRead, FromReadVar};

use std::io::Read;

//--------------------------------------------------------------------------------------------------

/// Ptvoice waveform composed of either coordinate points or sine overtones.
pub enum PtvWave {
    Coordinate {
        /// Points `(x, y)` that make up the waveform.
        points: Box<[(u8, i8)]>,
        /// x-width of the waveform.
        x_width: i32,
    },
    Oscillator {
        /// Overtone numbers and corresponding amplitudes that make up the waveform.
        overtones: Box<[(i32, i32)]>,
    },
}

impl PtvWave {
    const COORDINATE: i32 = 0;
    const OSCILLATOR: i32 = 1;
}

impl FromRead<Self> for PtvWave {
    type Error = PtvError;

    fn from_read<R: Read>(source: &mut R) -> Result<Self, Self::Error> {
        // Read & match wave type.
        match i32::from_read_var(source)? {
            Self::COORDINATE => {
                let point_count =
                    usize::try_from(i32::from_read_var(source)?).map_err(|_| PtvError::Invalid)?;
                let x_width = i32::from_read_var(source)?;

                // Read `(x, y)` pairs.
                let points = (0..point_count)
                    .map(|_| <(u8, i8)>::from_read(source))
                    .collect::<Result<Box<[_]>, _>>()?;

                Ok(PtvWave::Coordinate { points, x_width })
            }

            Self::OSCILLATOR => {
                let overtone_count =
                    usize::try_from(i32::from_read_var(source)?).map_err(|_| PtvError::Invalid)?;

                // Read `(overtone_num, amplitude)` pairs.
                let overtones = (0..overtone_count)
                    .map(|_| <(i32, i32)>::from_read_var(source))
                    .collect::<Result<Box<[_]>, _>>()?;

                Ok(Self::Oscillator { overtones })
            }

            // Unknown wave type.
            _ => Err(PtvError::Invalid),
        }
    }
}
