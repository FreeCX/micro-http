use std::fmt;

pub enum StatusCode {
    Ok,               // 200
    BadRequest,       // 400
    NotFound,         // 404
    MethodNotAllowed, // 405
    ServerError,      // 500
}

impl fmt::Display for StatusCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use StatusCode::*;
        write!(
            f,
            "{}",
            match self {
                Ok => "200 OK",
                BadRequest => "400 Bad Request",
                NotFound => "404 Not Found",
                MethodNotAllowed => "405 Method Not Allowed",
                ServerError => "500 Internal Server Error",
            }
        )
    }
}
