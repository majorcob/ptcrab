use super::{PtvEnvelope, PtvError, PtvWave};
use crate::data::{FromRead, FromReadVar, WriteTo, WriteVarTo};
use crate::voice::VoiceFlags;
use crate::{Key, PanVolume, Tuning, Volume};

use std::io::{Read, Seek, Write};

//--------------------------------------------------------------------------------------------------

/// Single ptvoice "channel" with its own waveform, envelope, and parameters.
#[derive(Clone, Debug, PartialEq)]
pub struct PtvUnit {
    /// Since ptvoices don't really have an inherent pitch (as opposed to samples), this is just set
    /// to A<sub>6</sub> (basic key `0x2D00`) by default. This can be changed to create harmonic
    /// intervals between units in a single ptvoice.
    pub inherent_key: Key,
    /// Overall volume for this unit.
    pub volume: Volume,
    /// Relative stereo channel volumes for this unit.
    pub pan_volume: PanVolume,
    /// Tuning value for this unit.
    pub tuning: Tuning,

    /// Unit voice flags.
    pub flags: VoiceFlags,
    /// Unit waveform. pxtone technically allows this to be absent, but tends to crash or behave
    /// unexpectedly. Omit with caution.
    pub wave: Option<PtvWave>,
    /// Unit envelope. If absent, volume will remain constant throughout a note's duration.
    pub envelope: Option<PtvEnvelope>,
}

impl PtvUnit {
    const HAS_WAVE: u32 = 1 << 0;
    const HAS_ENVELOPE: u32 = 1 << 1;
    const RESERVED: u32 = !(Self::HAS_WAVE | Self::HAS_ENVELOPE);
}

impl FromRead<Self> for PtvUnit {
    type Error = PtvError;

    fn from_read<R: Read>(source: &mut R) -> Result<Self, Self::Error> {
        let inherent_key = i32::from_read_var(source).map(Key::from_basic)?;
        let volume = i32::from_read_var(source)?.into();
        let pan_volume = i32::from_read_var(source)?.into();
        let tuning = f32::from_read_var(source)?.into();

        let flags = VoiceFlags::from_read_var(source)?.ok_or(PtvError::Invalid)?;

        // Wave & envelope data if present.
        let data_flags = u32::from_read_var(source)?;
        if (data_flags & Self::RESERVED) != 0 {
            return Err(PtvError::Invalid);
        }
        let wave = ((data_flags & Self::HAS_WAVE) != 0)
            .then(|| PtvWave::from_read(source))
            .transpose()?;
        let envelope = ((data_flags & Self::HAS_ENVELOPE) != 0)
            .then(|| PtvEnvelope::from_read(source))
            .transpose()?;

        Ok(Self {
            inherent_key,
            volume,
            pan_volume,
            tuning,

            flags,
            wave,
            envelope,
        })
    }
}

impl WriteTo for PtvUnit {
    type Error = PtvError;

    fn write_to<W: Write + Seek>(&self, sink: &mut W) -> Result<u64, Self::Error> {
        let start_pos = self.inherent_key.as_basic().write_var_to(sink)?;
        self.volume.as_value().write_var_to(sink)?;
        self.pan_volume.as_value().write_var_to(sink)?;
        f32::from(self.tuning).write_var_to(sink)?;

        self.flags.write_var_to(sink)?;

        // Wave & envelope data if present.
        let mut data_flags = 0_u32;
        if self.wave.is_some() {
            data_flags |= Self::HAS_WAVE;
        }
        if self.envelope.is_some() {
            data_flags |= Self::HAS_ENVELOPE;
        }
        data_flags.write_var_to(sink)?;
        if let Some(wave) = &self.wave {
            wave.write_to(sink)?;
        }
        if let Some(envelope) = &self.envelope {
            envelope.write_to(sink)?;
        }

        Ok(start_pos)
    }
}

impl Default for PtvUnit {
    fn default() -> Self {
        Self {
            inherent_key: Key::A6,
            volume: Volume::from(64),
            pan_volume: PanVolume::CENTER,
            tuning: Tuning::default(),

            flags: VoiceFlags {
                wave_loop: true,
                smooth: true,
                beat_fit: false,
            },

            wave: Some(PtvWave::default()),
            envelope: None,
        }
    }
}
