/// Multiplier applied to a unit's pitch.
///
/// Negative values will cause crashes or other unpleasant behaviour in pxtone, so use with caution.
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(transparent)]
pub struct Tuning(f32);

impl From<f32> for Tuning {
    fn from(value: f32) -> Self {
        Self(value)
    }
}
impl From<Tuning> for f32 {
    fn from(value: Tuning) -> Self {
        value.0
    }
}

impl Default for Tuning {
    fn default() -> Self {
        Self::from(1.)
    }
}
