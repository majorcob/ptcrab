use super::PtvError;
use crate::data::{FromRead, FromReadVar};
use crate::value::MaxLenI32;
use crate::MaxLen;

use std::io::Read;

//--------------------------------------------------------------------------------------------------

/// A ptvoice waveform, composed of either coordinate points or sine overtones.
pub enum PtvWave {
    Coordinate {
        /// Points `(x, y)` that make up the waveform.
        points: MaxLenI32<(u8, i8)>,
        /// x-width of the waveform.
        x_width: i32,
    },
    Oscillator {
        /// Overtone numbers and corresponding amplitudes that make up the waveform.
        overtones: MaxLenI32<(i32, i32)>,
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

                let mut points = vec![];
                for _ in 0..point_count {
                    let x = u8::from_read(source)?;
                    let y = i8::from_read(source)?;
                    points.push((x, y));
                }
                // SANITY: Point count already verified to be between 0 and `i32::MAX`.
                let points = MaxLenI32::new(points.into_boxed_slice()).unwrap();

                Ok(PtvWave::Coordinate { points, x_width })
            }

            Self::OSCILLATOR => {
                let overtone_count =
                    usize::try_from(i32::from_read_var(source)?).map_err(|_| PtvError::Invalid)?;

                let mut overtones = vec![];
                for _ in 0..overtone_count {
                    let index = i32::from_read_var(source)?;
                    let amplitude = i32::from_read_var(source)?;
                    overtones.push((index, amplitude));
                }
                let overtones = MaxLenI32::new(overtones.into_boxed_slice()).unwrap();

                Ok(Self::Oscillator { overtones })
            }

            // Unknown wave type.
            _ => Err(PtvError::Invalid),
        }
    }
}
