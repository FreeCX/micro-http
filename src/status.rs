use core::fmt;

pub enum StatusCode {
    Ok,          // 200
    BadRequest,  // 400
    NotFound,    // 404
    ServerError, // 500
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
                ServerError => "500 Internal Server Error",
            }
        )
    }
}
