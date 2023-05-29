use crate::http;
use crate::status;
use std::fs;

fn detect_content_type(filename: &str) -> String {
    // в идеале тут нужно определять mime-type файла по его содержимому, но нам хватит и этого набора
    match filename.rsplit_once('.') {
        Some((_, "css")) => "text/css",
        Some((_, "html")) => "text/html",
        Some((_, "ico")) => "image/x-icon",
        Some((_, "js")) => "text/javascript",
        Some((_, "png")) => "image/x-png",
        // любые другие файлы будут считаться просто бинарными данными
        _ => "application/octet-stream",
    }
    .to_string()
}

pub fn response(filename: &str) -> http::HttpData {
    match fs::read(filename) {
        Ok(content) => http::HttpData::from_content(detect_content_type(filename), content),
        Err(_) => http::HttpData::from_status(status::StatusCode::NotFound),
    }
}
