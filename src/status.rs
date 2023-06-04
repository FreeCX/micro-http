use std::fmt;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum StatusCode {
    Ok = 200,
    BadRequest = 400,
    NotFound = 404,
    MethodNotAllowed = 405,
    ServerError = 500,
}

impl fmt::Display for StatusCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use StatusCode::*;
        write!(
            f,
            "{} {}",
            *self as u16,
            match self {
                Ok => "OK",
                BadRequest => "Bad Request",
                NotFound => "Not Found",
                MethodNotAllowed => "Method Not Allowed",
                ServerError => "Internal Server Error",
            }
        )
    }
}
