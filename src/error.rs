#[derive(Debug)]
pub enum Error {
    RequestBodyMissing,
    RusqliteError(rusqlite::Error),
    IOError(std::io::Error),
    SerdeError(serde_json::Error),
}

impl From<rusqlite::Error> for Error {
    fn from(error: rusqlite::Error) -> Self {
        Error::RusqliteError(error)
    }
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Error::IOError(error)
    }
}

impl From<serde_json::Error> for Error {
    fn from(error: serde_json::Error) -> Self {
        Error::SerdeError(error)
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::RequestBodyMissing => write!(f, "request body is missing"),
            Error::RusqliteError(inner) => write!(f, "{}", inner),
            Error::IOError(inner) => write!(f, "{}", inner),
            Error::SerdeError(inner) => write!(f, "{}", inner),
        }
    }
}
