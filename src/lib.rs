//! A pure-Rust re-implementation of [the official pxtone library](https://pxtone.org/developer) with a safer, more intuitive API.
//!
//! ## Features
//!
//! - Support for both little- & big-endian systems
//! - Support for generic WASM targets (i.e. without Emscripten)
//! - I/O using the standard library's [`Read`](std::io::Read) & [`Write`](std::io::Write) traits
//! - Ptvoice data manipulation (as of v0.1)
//!
//! ### Not yet implemented
//!
//! - Ptnoise data manipulation
//! - Project data (ptcop/pttune) manipulation
//! - Audio rendering/playback
//!
//! ## License
//!
//! [MIT](https://choosealicense.com/licenses/mit/)

mod value;
pub use self::value::*;

pub mod data;
pub mod ptvoice;
pub mod voice;

//--------------------------------------------------------------------------------------------------

/// Re-exports for common use cases.
pub mod prelude {
    pub use super::data::{FromRead, WriteTo};
    pub use super::ptvoice::{PtvEnvelope, PtvError, PtvUnit, PtvWave, Ptvoice};
    pub use super::{Key, PanVolume, Tuning, Volume};
}
