//! Client errors, with one-hopes-helpful messages.

use thiserror::Error;

/// A convenient alias for the error type used by all crate functions.
pub type NuclinoResult<T> = Result<T, NuclinoError>;

/// Errors returned by this crate's functions. These include errors
/// derived from serde_json and ureq as well as errors representing
/// failure responses from the Nuclino API.
#[derive(Error, Debug)]
pub enum NuclinoError {
    /// Api key env var was required, but not found.
    #[error("Cannot find an API key in the process environment.")]
    ApiKeyNotFound,
    /// The Nuclino API reported a 4xx error in the client's request.
    #[error("Client error: status={status}; {message}")]
    ClientError {
        /// http status code
        status: u16,
        /// the message Nuclino included with the error
        message: String,
    },
    /// The Nuclino API reported an error on its own side (5xx).
    #[error("Nuclino service error: status={status}; {message}")]
    ServerError {
        /// the http status code
        status: u16,
        /// the message Nuclino included with the error
        message: String,
    },
    /// An error coming from the ureq crate.
    #[error("ureq reports error: {0}")]
    RequestError(String),
    /// An IO error.
    #[error(transparent)]
    IoError(#[from] std::io::Error),
    /// An error in serializing or deserializing json.
    #[error(transparent)]
    JsonError(#[from] serde_json::Error),
    /// A successful response from Nuclino did not include a data field in its wrapper.
    #[error("Didn't get a data field on the response")]
    NoDataReturned,
    /// The author of this crate made an error. Please report this as a bug.
    #[error("Programmer error. Please file a bug.")]
    ProgrammerError,
}

impl From<ureq::Error> for NuclinoError {
    fn from(value: ureq::Error) -> Self {
        NuclinoError::RequestError(value.to_string())
    }
}

/// An internal convenience for making Nuclino API responses into errors.
pub fn make_error(status: u16, message: String) -> NuclinoError {
    if status < 500 {
        NuclinoError::ClientError { status, message }
    } else {
        NuclinoError::ServerError { status, message }
    }
}
