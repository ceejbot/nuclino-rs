//! These types are used to deserialize Nuclino's response wrappers, and are not
//! types you're likely to need to use directly.

use serde::Deserialize;

/// The wrapper around all responses returned by the Nuclino API.
#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "status", rename_all = "camelCase")]
pub struct Response<T>
where
    T: Clone,
{
    status: String,
    message: Option<String>,
    data: Option<T>,
}

/// A trait shared by all responses from the Nuclino API.
pub trait ResponseInfo {
    type D;

    /// Get this response's error message if it has one.
    fn message(&self) -> String;
    /// Check if this request was successful.
    fn is_success(&self) -> bool;
    /// Check if this response reports a server error.
    fn is_server_error(&self) -> bool;
    /// Check if this response reports an error in the client's request.
    fn is_client_error(&self) -> bool;
    /// The data payload of this response, if one exists.
    fn data(&self) -> Option<&Self::D>;
}

impl<T> ResponseInfo for Response<T>
where
    T: Clone,
{
    type D = T;

    fn message(&self) -> String {
        if let Some(msg) = self.message.clone() {
            msg
        } else {
            "".to_string()
        }
    }

    fn data(&self) -> Option<&Self::D> {
        self.data.as_ref()
    }

    fn is_success(&self) -> bool {
        self.status == "success"
    }

    fn is_server_error(&self) -> bool {
        self.status == "error"
    }

    fn is_client_error(&self) -> bool {
        self.status == "fail"
    }
}

/// A list response structure, returned by any endpoint that responds
/// with a list of any kind. You probably won't need to use this type
/// directly, because the client functions return vectors.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct List<T: Clone> {
    pub results: Vec<T>,
}

impl<T> List<T>
where
    T: Clone,
{
    pub fn slice(&self) -> &[T] {
        self.results.as_slice()
    }

    pub fn as_vec(&self) -> Vec<T> {
        self.results.clone()
    }
}
