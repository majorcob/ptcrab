use std::io::Error as IoError;
use thiserror::Error as ThisError;

//--------------------------------------------------------------------------------------------------

/// Errors arising from ptvoice operations.
#[derive(Debug, ThisError)]
pub enum PtvError {
    /// Ptvoice has newer version than supported.
    #[error("unsupported ptvoice format")]
    Unsupported,
    /// Ptvoice data is malformed or contains an illegal value.
    #[error("invalid ptvoice data")]
    Invalid,
    /// Ptvoice data has too many items or is too long to encode its own length.
    #[error("ptvoice data exceeds maximum")]
    OverMax,

    /// I/O error while reading/writing ptvoice data.
    #[error("ptvoice I/O failure: {0}")]
    IoFailure(IoError),
}

impl From<IoError> for PtvError {
    fn from(value: IoError) -> Self {
        Self::IoFailure(value)
    }
}
