use std::fmt;

/// Error type for the semanticscholar crate.
#[derive(Debug)]
pub enum Error {
    /// The JSON value was not an object when one was expected.
    InvalidJson(String),
    /// The API returned an error message.
    Api(String),
    /// An HTTP request failed.
    Http(reqwest::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::InvalidJson(msg) => write!(f, "Invalid JSON: {msg}"),
            Error::Api(msg) => write!(f, "API error: {msg}"),
            Error::Http(err) => write!(f, "HTTP error: {err}"),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::Http(err) => Some(err),
            _ => None,
        }
    }
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        Error::Http(err)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_invalid_json_display() {
        let err = Error::InvalidJson("not an object".to_string());
        assert!(err.to_string().contains("not an object"));
    }

    #[test]
    fn test_api_error_display() {
        let err = Error::Api("paper not found".to_string());
        assert!(err.to_string().contains("paper not found"));
    }

    #[test]
    fn test_error_is_send_and_sync() {
        fn assert_send<T: Send>() {}
        fn assert_sync<T: Sync>() {}
        assert_send::<Error>();
        assert_sync::<Error>();
    }

    #[test]
    fn test_error_debug() {
        let err = Error::InvalidJson("test".to_string());
        let debug = format!("{:?}", err);
        assert!(debug.contains("InvalidJson"));
    }

    #[test]
    fn test_invalid_json_source_is_none() {
        let err = Error::InvalidJson("test".to_string());
        assert!(std::error::Error::source(&err).is_none());
    }

    #[test]
    fn test_api_error_source_is_none() {
        let err = Error::Api("test".to_string());
        assert!(std::error::Error::source(&err).is_none());
    }
}
