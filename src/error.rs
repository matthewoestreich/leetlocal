use std::fmt;

#[derive(Debug)]
pub enum Error {
    Network(reqwest::Error),
    DiskIo(std::io::Error),
    SerdeJSON(serde_json::Error),
    QuestionNotFound,
}

// 1. Implement Display for printing errors
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Network(err) => write!(f, "Network error: {}", err),
            Error::DiskIo(err) => write!(f, "I/O error: {}", err),
            Error::SerdeJSON(err) => write!(f, "Serde JSON error: {}", err),
            Error::QuestionNotFound => write!(f, "Question not found"),
        }
    }
}

// 2. Implement the standard Error trait
impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::Network(err) => Some(err),
            Error::DiskIo(err) => Some(err),
            Error::SerdeJSON(err) => Some(err),
            _ => None,
        }
    }
}

// 3. Implement From to use the `?` operator on reqwest calls
impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        Error::Network(err)
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Error::SerdeJSON(err)
    }
}
