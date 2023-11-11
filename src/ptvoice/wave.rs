use super::PtvError;
use crate::data::{FromRead, FromReadVar, WriteTo, WriteVarTo};

use std::io::{Read, Seek, Write};

//--------------------------------------------------------------------------------------------------

/// Ptvoice waveform composed of either coordinate points or sine overtones.
#[derive(Clone, Debug, PartialEq)]
pub enum PtvWave {
    Coordinate {
        /// Points `(x, y)` that make up the waveform.
        points: Box<[(u8, i8)]>,
        /// x-width of the waveform. Usually 256.
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

    /// Creates a coordinate waveform from the given points using the default x-width of 256.
    pub fn new_coordinate(points: Box<[(u8, i8)]>) -> Self {
        Self::Coordinate {
            points,
            x_width: 256,
        }
    }
    /// Creates an oscillator waveform from the given `(overtone_num, amplitude)` pairs.
    pub fn new_oscillator(overtones: Box<[(i32, i32)]>) -> Self {
        Self::Oscillator { overtones }
    }

    /// Returns a default sine waveform.
    pub fn default_sine() -> Self {
        Self::new_oscillator(Box::new([(1, 128)]))
    }
    /// Returns a default triangle waveform.
    pub fn default_triangle() -> Self {
        Self::new_coordinate(Box::new([(0, 0), (64, 64), (192, -64)]))
    }
    /// Returns a default sawtooth waveform.
    pub fn default_sawtooth() -> Self {
        Self::new_coordinate(Box::new([(0, 0), (0, 32), (255, -32)]))
    }
    /// Returns a default square waveform.
    pub fn default_square() -> Self {
        Self::new_coordinate(Box::new([
            (0, 0),
            (0, 32),
            (128, 32),
            (128, -32),
            (255, -32),
        ]))
    }
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

impl WriteTo for PtvWave {
    type Error = PtvError;

    fn write_to<W: Write + Seek>(&self, sink: &mut W) -> Result<u64, Self::Error> {
        match &self {
            Self::Coordinate { points, x_width } => {
                let start_pos = Self::COORDINATE.write_var_to(sink)?;

                i32::try_from(points.len())
                    .map_err(|_| PtvError::OverMax)?
                    .write_var_to(sink)?;
                x_width.write_var_to(sink)?;

                // Write `(x, y)` pairs.
                for (x, y) in points.iter() {
                    x.write_to(sink)?;
                    y.write_to(sink)?;
                }

                Ok(start_pos)
            }

            Self::Oscillator { overtones } => {
                let start_pos = Self::OSCILLATOR.write_var_to(sink)?;

                i32::try_from(overtones.len())
                    .map_err(|_| PtvError::OverMax)?
                    .write_var_to(sink)?;

                // Write `(overtone_num, amplitude)` pairs.
                for (overtone_num, amplitude) in overtones.iter() {
                    overtone_num.write_var_to(sink)?;
                    amplitude.write_var_to(sink)?;
                }

                Ok(start_pos)
            }
        }
    }
}

impl Default for PtvWave {
    fn default() -> Self {
        Self::new_coordinate(Box::new([(0, 0)]))
    }
}
