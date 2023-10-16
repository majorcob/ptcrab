use crate::voice::VoiceFlags;

/// Single ptvoice "channel" with its own waveform, parameters, and envelope.
pub struct PtvUnit {
    pub basic_key: i32,
    pub tuning: f32,
    pub volume: i32,
    pub pan_volume: i32,

    pub voice_flags: VoiceFlags,
    pub wave: Option<()>,     // TODO
    pub envelope: Option<()>, // TODO
}
