//! A pure-Rust alternative to [the official pxtone library](https://pxtone.org/developer/).

pub mod data;
pub mod ptvoice;
pub mod voice;

/// Re-exports for common use cases.
pub mod prelude {
    pub use super::data::{FromRead, WriteTo};
}
