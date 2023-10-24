use crate::data::{FromRead, FromReadVar, WriteTo, WriteVarTo};

use std::io::{Error as IoError, Read, Seek, Write};

/// Voice-unit configuration.
pub struct VoiceFlags {
    /// Repeats the waveform for the full note duration (vs. being a one-off sample).
    pub wave_loop: bool,
    /// Adds a very slight fadeout on note release.
    pub smooth: bool,
    /// Stretches the sample to be one beat in length, regardless of note duration.
    pub beat_fit: bool,
}

impl VoiceFlags {
    const WAVE_LOOP: u32 = 1 << 0;
    const SMOOTH: u32 = 1 << 1;
    const BEAT_FIT: u32 = 1 << 2;
    const RESERVED: u32 = !(Self::WAVE_LOOP | Self::SMOOTH | Self::BEAT_FIT);

    fn from_u32(value: u32) -> Option<Self> {
        if (value & Self::RESERVED) != 0 {
            return None;
        }

        Some(Self {
            wave_loop: (value & Self::WAVE_LOOP) != 0,
            smooth: (value & Self::SMOOTH) != 0,
            beat_fit: (value & Self::BEAT_FIT) != 0,
        })
    }

    fn as_u32(&self) -> u32 {
        (if self.wave_loop { Self::WAVE_LOOP } else { 0 })
            | (if self.smooth { Self::SMOOTH } else { 0 })
            | (if self.beat_fit { Self::BEAT_FIT } else { 0 })
    }
}

impl FromRead<Option<Self>> for VoiceFlags {
    type Error = IoError;

    fn from_read<R: Read>(source: &mut R) -> Result<Option<Self>, Self::Error> {
        u32::from_read(source).map(Self::from_u32)
    }
}

impl FromReadVar<Option<Self>> for VoiceFlags {
    type Error = IoError;

    fn from_read_var<R: Read>(source: &mut R) -> Result<Option<Self>, Self::Error> {
        u32::from_read_var(source).map(Self::from_u32)
    }
}

impl WriteTo for VoiceFlags {
    type Error = IoError;

    fn write_to<W: Write + Seek>(&self, sink: &mut W) -> Result<u64, Self::Error> {
        self.as_u32().write_to(sink)
    }
}

impl WriteVarTo for VoiceFlags {
    type Error = IoError;

    fn write_var_to<W: Write + Seek>(&self, sink: &mut W) -> Result<u64, Self::Error> {
        self.as_u32().write_var_to(sink)
    }
}
