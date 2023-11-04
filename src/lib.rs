//! A pure-Rust alternative to [the official pxtone library](https://pxtone.org/developer/).

mod value;
pub use self::value::*;

pub mod data;
pub mod ptvoice;
pub mod voice;

//--------------------------------------------------------------------------------------------------

/// Re-exports for common use cases.
pub mod prelude {
    pub use super::data::{FromRead, WriteTo};
    pub use super::{Key, PanVolume, Tuning, Volume};
}
