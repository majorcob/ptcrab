/// Represents relative volume between stereo channels; 0 = full left, 64 = equal, 128 = full right.
///
/// Due to how pxtone calculates volume for each channel, values < 0 will invert and gradually
/// amplify the *right* channel, while values > 128 will do the same to the *left* channel.
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(transparent)]
pub struct PanVolume(i32);

impl PanVolume {
    /// Center (equal) panning.
    pub const CENTER: Self = Self(64);
    /// Full-left panning.
    pub const LEFT: Self = Self(0);
    /// Full-right panning.
    pub const RIGHT: Self = Self(128);

    /// Converts from separate left and right values out of 64.
    ///
    /// ```
    /// # use ptcrab::PanVolume;
    /// assert_eq!(PanVolume::LEFT,   PanVolume::from_separate(64, 0));
    /// assert_eq!(PanVolume::CENTER, PanVolume::from_separate(64, 64));
    /// assert_eq!(PanVolume::RIGHT,  PanVolume::from_separate(0,  64));
    /// ```
    pub const fn from_separate(left: i32, right: i32) -> Self {
        Self(if left < 64 { 128 - left } else { right })
    }
    /// Converts from left and right volume ratios.
    ///
    /// ```
    /// # use ptcrab::PanVolume;
    /// assert_eq!(PanVolume::LEFT,   PanVolume::from_ratios(1., 0.));
    /// assert_eq!(PanVolume::CENTER, PanVolume::from_ratios(1., 1.));
    /// assert_eq!(PanVolume::RIGHT,  PanVolume::from_ratios(0., 1.));
    /// ```
    pub fn from_ratios(left: f32, right: f32) -> Self {
        Self::from_separate((left * 64.) as i32, (right * 64.) as i32)
    }

    /// Returns panning as a single [`i32`].
    ///
    /// ```
    /// # use ptcrab::PanVolume;
    /// assert_eq!(PanVolume::LEFT.as_value(),   0);
    /// assert_eq!(PanVolume::CENTER.as_value(), 64);
    /// assert_eq!(PanVolume::RIGHT.as_value(),  128);
    /// ```
    pub const fn as_value(&self) -> i32 {
        self.0
    }
    /// Returns panning as left and right values out of 64.
    ///
    /// ```
    /// # use ptcrab::PanVolume;
    /// assert_eq!(PanVolume::LEFT.as_separate(),   (64, 0));
    /// assert_eq!(PanVolume::CENTER.as_separate(), (64, 64));
    /// assert_eq!(PanVolume::RIGHT.as_separate(),  (0,  64));
    /// ```
    pub fn as_separate(&self) -> (i32, i32) {
        (64.min(128 - self.0), 64.min(self.0))
    }
    /// Returns panning as left and right volume ratios.
    ///
    /// ```
    /// # use ptcrab::PanVolume;
    /// assert_eq!(PanVolume::LEFT.as_ratios(),   (1., 0.));
    /// assert_eq!(PanVolume::CENTER.as_ratios(), (1., 1.));
    /// assert_eq!(PanVolume::RIGHT.as_ratios(),  (0., 1.));
    /// ```
    pub fn as_ratios(&self) -> (f32, f32) {
        let (left, right) = self.as_separate();

        ((left as f32) / 64., (right as f32) / 64.)
    }
}

impl From<i32> for PanVolume {
    fn from(value: i32) -> Self {
        Self(value)
    }
}

impl Default for PanVolume {
    fn default() -> Self {
        Self::CENTER
    }
}
