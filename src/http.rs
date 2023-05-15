use crate::read;
use crate::status;
use std::fmt;
use std::{collections::HashMap, io::Read, net::SocketAddr};

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum HttpMethod {
    GET,
    POST,
}

pub struct HttpData {
    pub headers: HashMap<String, String>,
    pub content: Option<Vec<u8>>,
    pub addr: Option<SocketAddr>,
    pub url: String,
    pub method: Option<HttpMethod>,
    pub status_code: status::StatusCode,
}

impl Default for HttpData {
    fn default() -> Self {
        Self::new()
    }
}

impl HttpData {
    pub fn new() -> HttpData {
        HttpData {
            headers: HashMap::new(),
            content: None,
            url: String::new(),
            addr: None,
            method: None,
            status_code: status::StatusCode::Ok,
        }
    }

    pub fn parse<R: Read>(&mut self, r: &mut R) {
        self.parse_header(r);
        self.parse_content(r);
    }

    pub fn add_header<K, V>(&mut self, key: K, value: V)
    where
        K: fmt::Display,
        V: fmt::Display,
    {
        self.headers.insert(key.to_string(), value.to_string());
    }

    // TODO: нормально распарсить заголовок
    fn parse_header<R: Read>(&mut self, r: &mut R) {
        let buffer = read::read_until_crlf(r).unwrap();
        let mut iterator = buffer.split("\r\n");

        let header: Vec<_> = iterator.next().unwrap().split(' ').collect();
        self.method = Some(HttpMethod::from(header[0]));
        self.url = header[1].to_string();

        for line in iterator {
            if line.trim().is_empty() {
                continue;
            }
            let index = line.find(':').unwrap();
            let (key, value) = line.split_at(index);
            self.headers.insert(key.trim().to_string(), value[1..].trim().to_string());
        }
    }

    fn parse_content<R: Read>(&mut self, r: &mut R) {
        if self.headers.contains_key("Content-Length") {
            let size: usize = self.headers.get("Content-Length").unwrap().parse().unwrap();
            let mut content = String::with_capacity(size);
            let r = Read::by_ref(r);
            let _ = r.take(size as u64).read_to_string(&mut content);
            self.content = Some(content.into());
        }
    }

    pub fn render_headers(&self) -> String {
        let mut buf = String::new();
        for (k, v) in &self.headers {
            buf.push_str(&format!("{k}: {v}\r\n"));
        }
        buf
    }

    pub fn render_content(&self) -> Vec<u8> {
        if let Some(data) = &self.content {
            data.clone()
        } else {
            Vec::new()
        }
    }
}

impl From<&str> for HttpMethod {
    fn from(value: &str) -> Self {
        match value {
            "GET" => HttpMethod::GET,
            "POST" => HttpMethod::POST,
            method => panic!("method {method} not supported"),
        }
    }
}
