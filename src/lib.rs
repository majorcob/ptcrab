//! A pure-Rust alternative to [the official pxtone library](https://pxtone.org/developer).
//!
//! ## Features
//!
//! - Support for systems of any endianness
//! - Support for generic Wasm targets
//! - I/O using standard [`Read`](std::io::Read) & [`Write`](std::io::Write) traits
//! - Ptvoice data manipulation
//!
//! ### Not yet implemented
//!
//! - Ptnoise data manipulation
//! - Project (ptcop/pttune) data manipulation
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
