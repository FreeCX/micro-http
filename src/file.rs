use crate::http;
use crate::status;
use std::fs;

fn detect_content_type(filename: &str) -> String {
    match filename.rsplit_once('.') {
        Some((_, "css")) => "text/css",
        Some((_, "html")) => "text/html",
        Some((_, "ico")) => "image/x-icon",
        Some((_, "js")) => "text/javascript",
        Some((_, "png")) => "image/x-png",
        _ => "text/plain",
    }
    .to_string()
}

pub fn response(filename: &str) -> http::HttpData {
    let mut response = http::HttpData::new();

    match fs::read(filename) {
        Ok(content) => {
            response.add_header("Content-Type", detect_content_type(filename));
            response.add_header("Content-Length", content.len());
            response.content = Some(content);
        }
        Err(_) => {
            response.status_code = status::StatusCode::NotFound;
        }
    }

    response
}
