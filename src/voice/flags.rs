/// Voice-unit configuration.
pub struct VoiceFlags {
    /// Repeats the waveform for the full note duration (vs. being a one-off sample).
    pub wave_loop: bool,
    /// Adds a very slight fadeout on note release.
    pub smooth: bool,
    /// Stretches the sample to be one beat in length, regardless of note duration.
    pub beat_fit: bool,
}
