use std::{any, error::Error};

use thiserror::Error;

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum BodyError {
    #[error("failed to serialize body to JSON: {}", source)]
    Json { source: serde_json::Error },
}

/// Errors which may occur when using API endpoints.
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum ApiError<E>
where
    E: Error + Send + Sync + 'static,
{
    /// The client encountered an error.
    #[error("client error: {}", source)]
    Client {
        /// The client error.
        source: E,
    },
    /// The URL failed to parse.
    #[error("failed to parse url: {}", source)]
    UrlParse {
        /// The source of the error.
        #[from]
        source: url::ParseError,
    },
    /// Body data could not be created.
    #[error("failed to create form data: {}", source)]
    Body {
        /// The source of the error.
        #[from]
        source: BodyError,
    },
    /// JSON deserialization from Teamdeck failed.
    #[error("could not parse JSON response: {}", source)]
    Json {
        /// The source of the error.
        #[from]
        source: serde_json::Error,
    },
    /// Failed to parse an expected data type from JSON.
    #[error("could not parse {} data from JSON: {}", typename, source)]
    DataType {
        /// The source of the error.
        source: serde_json::Error,
        /// The name of the type that could not be deserialized.
        typename: &'static str,
    },
    /// Teamdeck returned an error message.
    #[error("Teamdeck server error: {}", msg)]
    Teamdeck {
        /// The error message from Teamdeck.
        msg: String,
    },
    /// Teamdeck returned an error without JSON information.
    #[error("Teamdeck internal server error {}", status)]
    TeamdeckService {
        /// The status code for the return.
        status: http::StatusCode,
        /// The error data from Teamdeck.
        data: Vec<u8>,
    },

    /// Teamdeck returned an error object.
    #[error("teamdeck server error: {:?}", obj)]
    TeamdeckObject {
        /// The error object from Teamdeck.
        obj: serde_json::Value,
    },
    /// Teamdeck returned an HTTP error with JSON we did not recognize.
    #[error("Teamdeck server error: {:?}", obj)]
    TeamdeckUnrecognized {
        /// The full object from Teamdeck.
        obj: serde_json::Value,
    },
}

impl<E> ApiError<E>
where
    E: Error + Send + Sync + 'static,
{
    pub fn client(source: E) -> Self {
        ApiError::Client { source }
    }

    pub(crate) fn data_type<T>(source: serde_json::Error) -> Self {
        ApiError::DataType {
            source,
            typename: any::type_name::<T>(),
        }
    }

    pub(crate) fn from_teamdeck(value: serde_json::Value) -> Self {
        let error_value = value
            .pointer("/message")
            .or_else(|| value.pointer("/error"));

        if let Some(error_value) = error_value {
            if let Some(msg) = error_value.as_str() {
                ApiError::Teamdeck { msg: msg.into() }
            } else {
                ApiError::TeamdeckObject {
                    obj: error_value.clone(),
                }
            }
        } else {
            ApiError::TeamdeckUnrecognized { obj: value }
        }
    }

    pub(crate) fn server_error(status: http::StatusCode, body: &bytes::Bytes) -> Self {
        Self::TeamdeckService {
            status,
            data: body.into_iter().copied().collect(),
        }
    }
}
