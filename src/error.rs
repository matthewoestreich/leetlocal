use crate::Language;
use std::fmt;

#[allow(dead_code)]
#[derive(Debug)]
pub enum Error {
    Network(reqwest::Error),
    DiskIo(std::io::Error),
    SerdeJSON(serde_json::Error),
    QuestionNotFound,
    OutputDirectoryNotDirectory,
    OutputDirectoryNotFound {
        output_path: String,
    },
    UnsupportedLanguage(Language),
    DescriptionFileExists {
        description_file_name: String,
    },
    CodeFileExists {
        code_file_name: String,
        code_file_extension: String,
    },
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Network(err) => write!(f, "Network error: {}", err),
            Error::DiskIo(err) => write!(f, "I/O error: {}", err),
            Error::SerdeJSON(err) => write!(f, "Serde JSON error: {}", err),
            Error::QuestionNotFound => write!(f, "Question not found"),
            Error::UnsupportedLanguage(lang) => {
                write!(f, "Question does not support language '{lang}'")
            }
            Error::OutputDirectoryNotFound { output_path } => write!(
                f,
                "Output directory '{output_path}' does not exist and '--force' was not used! Please use '--force' to automatically create output pathway."
            ),
            Error::OutputDirectoryNotDirectory => {
                write!(f, "Output directory must be to a directory!")
            }
            Error::DescriptionFileExists {
                description_file_name,
            } => {
                write!(
                    f,
                    "{description_file_name}.md already exists at output directory. Please use '--force' to overwrite files!"
                )
            }
            Error::CodeFileExists {
                code_file_name,
                code_file_extension,
            } => {
                write!(
                    f,
                    "{code_file_name}.{code_file_extension} already exists at output directory. Please use '--force' to overwrite files!"
                )
            }
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

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::DiskIo(err)
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
