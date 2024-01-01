use std::ops::Mul;

//--------------------------------------------------------------------------------------------------

/// Volume ratio where a value of 128 = 100% volume.
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(transparent)]
pub struct Volume(i32);

impl Volume {
    /// Converts from volume ratio.
    pub fn from_ratio(ratio: f32) -> Self {
        Self::from((128. * ratio) as i32)
    }

    /// Returns volume value as [`i32`].
    pub const fn as_value(&self) -> i32 {
        self.0
    }
    /// Returns volume as a ratio.
    pub fn as_ratio(&self) -> f32 {
        (self.0 as f32) / 128.
    }
}

impl From<i32> for Volume {
    fn from(value: i32) -> Self {
        Self(value)
    }
}

impl Default for Volume {
    fn default() -> Self {
        Self::from(128)
    }
}

impl Mul<f32> for Volume {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self::from_ratio(self.as_ratio() * rhs)
    }
}
