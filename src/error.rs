use std::fmt;

#[allow(dead_code)]
#[derive(Debug)]
pub enum Error {
    Network(reqwest::Error),
    DiskIo(std::io::Error),
    SerdeJSON(serde_json::Error),
    QuestionNotFound,
}

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
