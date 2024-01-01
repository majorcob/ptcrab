use super::PtvError;
use crate::data::{FromRead, FromReadVar, WriteTo, WriteVarTo};

use std::io::{Read, Seek, Write};

//--------------------------------------------------------------------------------------------------

/// Ptvoice envelope defined by a sequence of points `(x, y)`, where the x-axis represents time (in
/// "ticks") and the y-axis represents volume.
#[derive(Clone, Debug, PartialEq)]
pub struct PtvEnvelope {
    /// List of absolute points `(x, y)` in the envelope. The last point in this list will be
    /// sustained while a note is held.
    pub points: Box<[(i32, i32)]>,
    /// Release duration in ticks.
    pub release: i32,
    /// Envelope tick rate. Usually set to 1000 so that 1 tick = 1 millisecond.
    pub ticks_per_second: i32,
}

impl PtvEnvelope {
    /// Creates a ptvoice envelope with the given points and release time, using the default tick
    /// rate per second of 1000.
    pub fn new(points: Box<[(i32, i32)]>, release: i32) -> Self {
        Self {
            points,
            release,
            ticks_per_second: 1000,
        }
    }
}

impl FromRead<Self> for PtvEnvelope {
    type Error = PtvError;

    fn from_read<R: Read>(source: &mut R) -> Result<Self, Self::Error> {
        let ticks_per_second = i32::from_read_var(source)?;
        let point_count: usize = i32::from_read_var(source)?
            .try_into()
            .map_err(|_| PtvError::Invalid)?;
        // Read unused point counts, verifying that their values are 0 and 1 respectively. These are
        // leftovers from when pxtone was planned to have separate attack, sustain, and release
        // envelopes; now, there must be exactly 0 sustain points and exactly 1 release point.
        if (i32::from_read_var(source)? != 0) || (i32::from_read_var(source)? != 1) {
            return Err(PtvError::Invalid);
        }

        // Read "attack" points, which should consist of every envelope point except the last.
        // x-values are relative to the previous point, so also track absolute x-value.
        let mut points = vec![];
        let mut prev_x = 0;
        for _ in 0..point_count {
            let (dx, y) = <(i32, i32)>::from_read_var(source)?;
            prev_x += dx;
            points.push((prev_x, y));
        }
        let points = points.into_boxed_slice();

        // Read single release point. pxtone hardcodes a 0 for the release y-value, so the y-value
        // obtained here goes unused.
        let (release, _release_y) = <(i32, i32)>::from_read_var(source)?;

        Ok(Self {
            points,
            release,
            ticks_per_second,
        })
    }
}

impl WriteTo for PtvEnvelope {
    type Error = PtvError;

    fn write_to<W: Write + Seek>(&self, sink: &mut W) -> Result<u64, Self::Error> {
        let start_pos = self.ticks_per_second.write_var_to(sink)?;
        i32::try_from(self.points.len())
            .map_err(|_| PtvError::Oversized)?
            .write_var_to(sink)?;
        // Unused sustain & release point counts, always expected to be 0 and 1.
        0_i32.write_var_to(sink)?;
        1_i32.write_var_to(sink)?;

        // Write points.
        let mut prev_x = 0;
        for &(x, y) in self.points.iter() {
            (x - prev_x, y).write_var_to(sink)?;
            prev_x = x;
        }
        // Write single release point.
        (self.release, 0_i32).write_var_to(sink)?;

        Ok(start_pos)
    }
}

impl Default for PtvEnvelope {
    fn default() -> Self {
        Self::new(Box::new([(0, 96)]), 1)
    }
}
