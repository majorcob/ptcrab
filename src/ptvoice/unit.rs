use super::{PtvEnvelope, PtvError, PtvWave};
use crate::data::{FromRead, FromReadVar};
use crate::voice::VoiceFlags;
use crate::{Key, PanVolume, Tuning, Volume};

use std::io::Read;

//--------------------------------------------------------------------------------------------------

/// Single ptvoice "channel" with its own waveform, parameters, and envelope.
pub struct PtvUnit {
    pub basic_key: Key,
    pub volume: Volume,
    pub pan_volume: PanVolume,
    pub tuning: Tuning,

    pub flags: VoiceFlags,
    pub wave: Option<PtvWave>,
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
        let basic_key = i32::from_read_var(source)?.into();
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
            basic_key,
            volume,
            pan_volume,
            tuning,

            flags,
            wave,
            envelope,
        })
    }
}
