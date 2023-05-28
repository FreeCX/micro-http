use crate::read;
use crate::status;
use std::fmt;
use std::{collections::HashMap, io::Read, net::SocketAddr};

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum HttpMethod {
    CONNECT,
    DELETE,
    GET,
    HEAD,
    OPTIONS,
    PATCH,
    POST,
    PUT,
    TRACE,
    UNKNOWN,
}

pub struct HttpData {
    // тут вообще multimap должен быть, но как-то пофиг пока
    pub headers: HashMap<String, String>,
    pub content: Option<Vec<u8>>,
    pub addr: Option<SocketAddr>,
    pub url: String,
    pub method: HttpMethod,
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
            method: HttpMethod::UNKNOWN,
            status_code: status::StatusCode::Ok,
        }
    }

    pub fn from(status: status::StatusCode) -> HttpData {
        let mut data = HttpData::new();
        data.status_code = status;
        data
    }

    pub fn parse<R: Read>(&mut self, r: &mut R) -> Option<()> {
        self.parse_header(r)?;
        self.parse_content(r)?;
        Some(())
    }

    pub fn add_header<K, V>(&mut self, key: K, value: V)
    where
        K: fmt::Display,
        V: fmt::Display,
    {
        // пусть все ключи будут в нижнем регистре
        self.headers.insert(key.to_string().to_lowercase(), value.to_string());
    }

    pub fn set_content<I: Into<Vec<u8>>>(&mut self, content: I) {
        let content = content.into();
        self.add_header("content-length", content.len());
        self.content = Some(content);
    }

    fn parse_header<R: Read>(&mut self, r: &mut R) -> Option<()> {
        let buffer = read::read_until_crlf(r)?;
        let mut iterator = buffer.split("\r\n");

        let header: Vec<_> = iterator.next()?.split(' ').collect();
        self.method = HttpMethod::from(header[0]);
        self.url = header[1].to_string();

        for line in iterator {
            if line.trim().is_empty() {
                continue;
            }
            let index = line.find(':')?;
            let (key, value) = line.split_at(index);
            self.headers.insert(key.trim().to_string().to_lowercase(), value[1..].trim().to_string());
        }

        Some(())
    }

    fn parse_content<R: Read>(&mut self, r: &mut R) -> Option<()> {
        if self.headers.contains_key("content-length") {
            let size: usize = self.headers.get("content-length")?.parse().ok()?;
            let mut content = String::with_capacity(size);
            let r = Read::by_ref(r);
            let _ = r.take(size as u64).read_to_string(&mut content);
            self.content = Some(content.into());
        }
        Some(())
    }

    pub fn render_headers(&self) -> String {
        let mut buf = String::new();
        for (k, v) in &self.headers {
            buf.push_str(&format!("{k}: {v}\r\n"));
        }
        buf
    }

    pub fn render_content(&self) -> Vec<u8> {
        self.content.clone().unwrap_or_default()
    }
}

impl From<&str> for HttpMethod {
    fn from(value: &str) -> Self {
        match value {
            "CONNECT" => HttpMethod::CONNECT,
            "DELETE" => HttpMethod::DELETE,
            "GET" => HttpMethod::GET,
            "HEAD" => HttpMethod::HEAD,
            "OPTIONS" => HttpMethod::OPTIONS,
            "PATCH" => HttpMethod::PATCH,
            "POST" => HttpMethod::POST,
            "PUT" => HttpMethod::PUT,
            "TRACE" => HttpMethod::TRACE,
            _ => HttpMethod::UNKNOWN,
        }
    }
}
