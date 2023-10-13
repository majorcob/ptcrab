//! (De)serializing pxtone data structures.
//!
//! Consider wrapping data sources with [`BufReader`](std::io::BufReader) and sinks with [`BufWriter`](std::io::BufWriter)
//! to avoid extraneous I/O calls.

mod from_read;
pub use self::from_read::*;
