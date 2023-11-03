use super::PtvError;
use crate::data::FromRead;
use crate::voice::VoiceFlags;
use crate::{Key, PanVolume, Tuning, Volume};

use std::io::Read;

//--------------------------------------------------------------------------------------------------

/// Single ptvoice "channel" with its own waveform, parameters, and envelope.
pub struct PtvUnit {
    pub basic_key: Key,
    pub tuning: Tuning,
    pub volume: Volume,
    pub pan_volume: PanVolume,

    pub flags: VoiceFlags,
    pub wave: Option<()>,     // TODO
    pub envelope: Option<()>, // TODO
}

impl FromRead<Self> for PtvUnit {
    type Error = PtvError;

    fn from_read<R: Read>(source: &mut R) -> Result<Self, Self::Error> {
        todo!()
    }
}
