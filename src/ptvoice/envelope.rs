use super::PtvError;
use crate::data::{FromRead, FromReadVar};

use std::io::{Error as IoError, Read};

//--------------------------------------------------------------------------------------------------

/// Ptvoice envelope defined by a sequence of points.
pub struct PtvEnvelope {
    /// List of absolute points `(x, y)` in the envelope. The last point in this list will be
    /// sustained while a note is held.
    pub points: Box<[(i32, i32)]>,
    /// Release duration, in the same units as envelope point x-values.
    pub release: i32,
    /// TODO: Envelope x-units per second?
    pub fps: i32,
}

impl FromRead<Self> for PtvEnvelope {
    type Error = PtvError;

    fn from_read<R: Read>(source: &mut R) -> Result<Self, Self::Error> {
        let fps = i32::from_read_var(source)?;
        let point_count =
            usize::try_from(i32::from_read_var(source)?).map_err(|_| PtvError::Invalid)?;
        // Read unused point counts, verifying that their values are 0 and 1 respectively. These are
        // leftovers from when pxtone was planned to have separate attack, sustain, and release
        // envelopes; now, there must be exactly 0 sustain points and exactly 1 release point.
        if (i32::from_read_var(source)? != 0) || (i32::from_read_var(source)? != 1) {
            return Err(PtvError::Invalid);
        }

        // Read "attack" points, which should consist of every envelope point except the last.
        // x-values are relative to the previous point, so also track absolute x-value.
        let points = (0..point_count)
            .try_fold((vec![], 0), |(mut points, prev_x), _| {
                let (dx, y) = <(i32, i32)>::from_read_var(source)?;
                let x = prev_x + dx;
                points.push((x, y));

                Ok::<(Vec<(i32, i32)>, i32), IoError>((points, x))
            })
            .map(|(points, _)| points.into_boxed_slice())?;

        // Read single release point. pxtone hardcodes a 0 for the release y-value, so the y-value
        // obtained here goes unused.
        let (release, _release_y) = <(i32, i32)>::from_read_var(source)?;

        Ok(Self {
            points,
            release,
            fps,
        })
    }
}
