//! (De)serializing pxtone data structures.
//!
//! Consider wrapping data sources with [`BufReader`](std::io::BufReader) and sinks with [`BufWriter`](std::io::BufWriter)
//! to avoid extraneous I/O calls.

mod from_read;
mod from_read_var;
mod write_to;
mod write_var_to;
pub use self::from_read::*;
pub use self::from_read_var::*;
pub use self::write_to::*;
pub use self::write_var_to::*;
