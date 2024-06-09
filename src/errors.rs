//! Client errors, with one-hopes-helpful messages.

use thiserror::Error;

pub type NuclinoResult<T> = Result<T, NuclinoError>;

#[derive(Error, Debug)]
pub enum NuclinoError {
    #[error("Cannot find an API key in the process environment.")]
    ApiKeyNotFound,
    #[error("Client error: status={status}; {message}")]
    ClientError { status: u16, message: String },
    #[error("Nuclino service error: status={status}; {message}")]
    ServerError { status: u16, message: String },
    #[error("ureq reports error: {0}")]
    RequestError(String),
    #[error(transparent)]
    IoError(#[from] std::io::Error),
    #[error(transparent)]
    JsonError(#[from] serde_json::Error),
    #[error("Programmer error")]
    ProgrammerError,
    #[error("Didn't get a data field on the response")]
    NoDataReturned,
}

impl From<ureq::Error> for NuclinoError {
    fn from(value: ureq::Error) -> Self {
        NuclinoError::RequestError(value.to_string())
    }
}

pub fn make_error(status: u16, message: String) -> NuclinoError {
    if status < 500 {
        NuclinoError::ClientError { status, message }
    } else {
        NuclinoError::ServerError { status, message }
    }
}
