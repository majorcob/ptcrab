//! General functionality for all voice types.

mod flags;
pub use self::flags::*;

use crate::ptvoice::Ptvoice;

//--------------------------------------------------------------------------------------------------

/// A single reusable sample or synthesized instrument.
pub enum Voice {
    Ptv(Ptvoice),
}

impl From<Ptvoice> for Voice {
    fn from(value: Ptvoice) -> Self {
        Self::Ptv(value)
    }
}
