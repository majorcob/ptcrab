use crate::voice::VoiceFlags;
use crate::Key;

/// Single ptvoice "channel" with its own waveform, parameters, and envelope.
pub struct PtvUnit {
    pub basic_key: Key,
    pub tuning: f32,
    pub volume: i32,
    pub pan_volume: i32,

    pub flags: VoiceFlags,
    pub wave: Option<()>,     // TODO
    pub envelope: Option<()>, // TODO
}
