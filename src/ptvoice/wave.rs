use super::PtvError;
use crate::data::{FromRead, FromReadVar, WriteTo, WriteVarTo};

use std::io::{Read, Seek, Write};

//--------------------------------------------------------------------------------------------------

/// Ptvoice waveform composed of either coordinate points or sine-wave harmonics.
#[derive(Clone, Debug, PartialEq)]
pub enum PtvWave {
    Coordinate {
        /// Points `(x, y)` that make up the waveform.
        ///
        /// The official ptvoice editor will refuse to open waveforms with more than 31 points,
        /// though pxtone seems to be able to handle as many as 254 points.
        points: Box<[(u8, i8)]>,
        /// x-width of the waveform. Usually 256.
        x_width: i32,
    },
    Oscillator {
        /// Harmonic numbers and corresponding amplitudes that make up the waveform.
        ///
        /// The "0th" harmonic is always silent, and negative harmonic numbers will crash pxtone.
        harmonics: Box<[(i32, i32)]>,
    },
}

impl PtvWave {
    const COORDINATE: i32 = 0;
    const OSCILLATOR: i32 = 1;

    /// Creates a coordinate waveform from the given points `(x, y)` using the default x-width of
    /// 256.
    pub fn coordinate_from_points(points: Box<[(u8, i8)]>) -> Self {
        Self::Coordinate {
            points,
            x_width: 256,
        }
    }
    /// Creates a coordinate waveform from a function f(x) = y over x = [0, 1) with the given number
    /// of points.
    ///
    /// The official ptvoice editor will refuse to open waveforms with more than 31 points, though
    /// pxtone seems to be able to handle as many as 254 points.
    pub fn coordinate_from_function(point_count: u8, y_function: impl Fn(f32) -> f32) -> Self {
        Self::coordinate_from_points(
            (0..point_count)
                .map(|i| {
                    let x = i as f32 / point_count as f32;

                    ((256. * x) as u8, (127. * y_function(x)) as i8)
                })
                .collect(),
        )
    }

    /// Creates an oscillator waveform from the given `(harmonic_num, amplitude)` pairs.
    ///
    /// The "0th" harmonic is always silent, and negative harmonic numbers will crash pxtone.
    pub fn oscillator_from_pairs(harmonics: Box<[(i32, i32)]>) -> Self {
        Self::Oscillator { harmonics }
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
                let harmonic_count =
                    usize::try_from(i32::from_read_var(source)?).map_err(|_| PtvError::Invalid)?;

                // Read `(harmonic_num, amplitude)` pairs.
                let harmonics = (0..harmonic_count)
                    .map(|_| <(i32, i32)>::from_read_var(source))
                    .collect::<Result<Box<[_]>, _>>()?;

                Ok(Self::Oscillator { harmonics })
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

            Self::Oscillator { harmonics } => {
                let start_pos = Self::OSCILLATOR.write_var_to(sink)?;

                i32::try_from(harmonics.len())
                    .map_err(|_| PtvError::OverMax)?
                    .write_var_to(sink)?;

                // Write `(harmonic_num, amplitude)` pairs.
                for (harmonic_num, amplitude) in harmonics.iter() {
                    harmonic_num.write_var_to(sink)?;
                    amplitude.write_var_to(sink)?;
                }

                Ok(start_pos)
            }
        }
    }
}

impl Default for PtvWave {
    fn default() -> Self {
        Self::coordinate_from_points(Box::new([(0, 0)]))
    }
}
