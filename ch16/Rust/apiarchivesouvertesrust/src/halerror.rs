use core::fmt;
use std::error;
use reqwest::Error as RequestError;
use serde_json::Error as JsonError;

#[derive(Debug)]
pub enum HALError {
    GenericError(String),
    RequestError(RequestError),
    JsonError(JsonError),
    
}

impl fmt::Display for HALError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use HALError::*;

        match self {
            GenericError(err) => err.fmt(f),
            RequestError(err) => err.fmt(f),
            JsonError(err) => err.fmt(f),
        }
    }
}

impl error::Error for HALError {
    fn description(&self) -> &str {
        use HALError::*;

        match self {
            GenericError(..) => "Erreur générique",
            RequestError(..) => "Erreur HTTP API",
            JsonError(..) => "Erreur Json",
        }
    }
}

#[doc(hidden)]
impl From<RequestError> for HALError {
    fn from(err: RequestError) -> Self {
        HALError::RequestError(err)
    }
}

#[doc(hidden)]
impl From<JsonError> for HALError {
    fn from(err: JsonError) -> Self {
        HALError::JsonError(err)
    }
}