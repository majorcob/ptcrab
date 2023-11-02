use crate::voice::VoiceFlags;
use crate::{Key, PanVolume, Tuning, Volume};

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
