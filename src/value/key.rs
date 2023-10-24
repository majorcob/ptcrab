/// Expression of pitch where 256 "key" increments = 1 semitone.
///
/// A value of 0 is equivalent to A<sub>(-4)</sub> (~1.72 Hz); "key" is thus represented as the
/// distance from that pitch 1/256th-semitone increments. For example, A<sub>4</sub> is 96 semitones
/// above A<sub>(-4)</sub>, so it is represented using a key value of 96 × 256 = `0x6000`.
///
/// pxtone sometimes refers to key relative to A<sub>4</sub> (instead of A<sub>(-4)</sub>) so that
/// commonly used pitches produce shorter encoded sequences.
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(transparent)]
pub struct Key(i32);

impl Key {
    /// A<sub>4</sub> (440 Hz) tone.
    pub const A4: Self = Self(96 * 256);
    /// Middle C tone.
    pub const C4: Self = Self(87 * 256);

    /// Converts from key relative to A<sub>4</sub>.
    ///
    /// ```
    /// # use ptcrab::Key;
    /// assert_eq!(Key::A4, Key::from_a4_offset(0));
    /// assert_eq!(Key::C4, Key::from_a4_offset(-9 * 256));
    /// ```
    pub fn from_a4_offset(a4_offset: i32) -> Self {
        Self::from(Self::A4.as_value() + a4_offset)
    }
    /// Approximates key based on distance in semitones from A<sub>(-4)</sub>.
    ///
    /// ```
    /// # use ptcrab::Key;
    /// assert_eq!(Key::A4, Key::approx_from_semis(96.));
    /// assert_eq!(Key::C4, Key::approx_from_semis(87.));
    /// ```
    pub fn approx_from_semis(semis: f32) -> Self {
        Self::from((semis * 256.) as i32)
    }
    /// Approximates key based on offset in semitones from A<sub>4</sub>.
    ///
    /// ```
    /// # use ptcrab::Key;
    /// assert_eq!(Key::A4, Key::approx_from_a4_semis(0.));
    /// assert_eq!(Key::C4, Key::approx_from_a4_semis(-9.));
    /// ```
    pub fn approx_from_a4_semis(a4_semis: f32) -> Self {
        Self::approx_from_semis(Self::A4.as_semis() + a4_semis)
    }
    /// Approximates key from frequency in hertz.
    ///
    /// ```
    /// # use ptcrab::Key;
    /// assert_eq!(Key::A4, Key::approx_from_hertz(440.));
    /// assert_eq!(Key::C4, Key::approx_from_hertz(261.62555));
    /// ```
    pub fn approx_from_hertz(frequency: f32) -> Self {
        Self::approx_from_a4_semis((frequency / Self::A4.as_hertz()).log2() * 12.)
    }

    /// Returns key value as [`i32`].
    ///
    /// ```
    /// # use ptcrab::Key;
    /// assert_eq!(Key::A4.as_value(), 96 * 256);
    /// assert_eq!(Key::C4.as_value(), 87 * 256);
    /// ```
    pub fn as_value(&self) -> i32 {
        self.0
    }
    /// Returns key relative to A<sub>4</sub>.
    ///
    /// ```
    /// # use ptcrab::Key;
    /// assert_eq!(Key::A4.as_a4_offset(), 0);
    /// assert_eq!(Key::C4.as_a4_offset(), -9 * 256);
    /// ```
    pub fn as_a4_offset(&self) -> i32 {
        self.as_value() - Self::A4.as_value()
    }
    /// Returns distance in semitones from A<sub>(-4)</sub>.
    ///
    /// ```
    /// # use ptcrab::Key;
    /// assert_eq!(Key::A4.as_semis(), 96.);
    /// assert_eq!(Key::C4.as_semis(), 87.);
    /// ```
    pub fn as_semis(&self) -> f32 {
        (self.as_value() as f32) / 256.
    }
    /// Returns offset in semitones from A<sub>4</sub>.
    ///
    /// ```
    /// # use ptcrab::Key;
    /// assert_eq!(Key::A4.as_a4_semis(), 0.);
    /// assert_eq!(Key::C4.as_a4_semis(), -9.);
    /// ```
    pub fn as_a4_semis(&self) -> f32 {
        (self.as_a4_offset() as f32) / 256.
    }
    /// Returns frequency in hertz.
    ///
    /// ```
    /// # use ptcrab::Key;
    /// assert_eq!(Key::A4.as_hertz(), 440.);
    /// assert_eq!(Key::C4.as_hertz(), 261.62555);
    /// ```
    pub fn as_hertz(&self) -> f32 {
        (self.as_a4_semis() / 12.).exp2() * 440.
    }
}

impl From<i32> for Key {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
