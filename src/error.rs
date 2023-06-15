use std::error;
use std::fmt;
use std::io;
use std::string;

#[derive(Debug)]
pub enum FrameworkError {
    Io(io::Error),
    Utf(string::FromUtf8Error),
    HeaderParse,
    HeaderData,
}

impl error::Error for FrameworkError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            FrameworkError::Io(e) => Some(e),
            FrameworkError::Utf(e) => Some(e),
            FrameworkError::HeaderParse => None,
            FrameworkError::HeaderData => None,
        }
    }
}

impl fmt::Display for FrameworkError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FrameworkError::Io(e) => write!(f, "IO Error: {}", e),
            FrameworkError::Utf(e) => write!(f, "UTF8 Error: {}", e),
            FrameworkError::HeaderParse => write!(f, "Parse header error"),
            FrameworkError::HeaderData => write!(f, "Get data from header error"),
        }
    }
}

impl From<io::Error> for FrameworkError {
    fn from(e: io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<string::FromUtf8Error> for FrameworkError {
    fn from(e: string::FromUtf8Error) -> Self {
        Self::Utf(e)
    }
}
