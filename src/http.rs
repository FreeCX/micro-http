use std::fmt;
use std::{collections::HashMap, io::Read, net::SocketAddr};

use crate::error::FrameworkError;
use crate::read;
use crate::status::StatusCode;

#[derive(Eq, PartialEq, Copy, Clone, Debug, Default)]
pub enum Method {
    CONNECT,
    DELETE,
    GET,
    HEAD,
    OPTIONS,
    PATCH,
    POST,
    PUT,
    TRACE,
    #[default]
    UNKNOWN,
}

#[derive(Default)]
pub struct Data {
    // тут вообще multimap должен быть, но как-то пофиг пока
    pub headers: HashMap<String, String>,
    pub content: Option<Vec<u8>>,
    pub addr: Option<SocketAddr>,
    pub url: String,
    pub method: Method,
    pub status_code: StatusCode,
}

impl Data {
    pub fn new() -> Data {
        Data::default()
    }

    pub fn from_status(status: StatusCode) -> Data {
        Data { status_code: status, ..Default::default() }
    }

    pub fn from_content<M: Into<String>, C: Into<Vec<u8>>>(mime_type: M, content: C) -> Data {
        let mut data = Data::new();
        let content = content.into();
        data.add_header("content-type", mime_type.into());
        data.add_header("content-length", content.len());
        data.content = Some(content);
        data
    }

    pub fn parse<R: Read>(&mut self, r: &mut R) -> Result<(), FrameworkError> {
        self.parse_header(r)?;
        self.parse_content(r)?;
        Ok(())
    }

    pub fn add_header<K, V>(&mut self, key: K, value: V)
    where
        K: fmt::Display,
        V: fmt::Display,
    {
        // пусть все ключи будут в нижнем регистре
        self.headers.insert(key.to_string().to_lowercase(), value.to_string());
    }

    fn parse_header<R: Read>(&mut self, r: &mut R) -> Result<(), FrameworkError> {
        let buffer = read::until_crlf(r)?;
        let mut iterator = buffer.split("\r\n");

        let header: Vec<_> = iterator.next().ok_or(FrameworkError::HeaderParse)?.split(' ').collect();
        self.method = Method::from(header[0]);
        self.url = header[1].to_string();

        for line in iterator {
            if line.trim().is_empty() {
                continue;
            }
            let index = line.find(':').ok_or(FrameworkError::HeaderParse)?;
            let (key, value) = line.split_at(index);
            self.add_header(key.trim(), value[1..].trim());
        }

        Ok(())
    }

    fn parse_content<R: Read>(&mut self, r: &mut R) -> Result<(), FrameworkError> {
        if self.headers.contains_key("content-length") {
            let size: u64 = self
                .headers
                .get("content-length")
                .ok_or(FrameworkError::HeaderData)?
                .parse()
                .map_err(|_| FrameworkError::HeaderData)?;
            let mut content = String::with_capacity(size as usize);
            let r = Read::by_ref(r);
            let _ = r.take(size).read_to_string(&mut content);
            self.content = Some(content.into());
        }
        Ok(())
    }

    pub fn render_headers(&self) -> String {
        let mut buf = String::new();
        for (k, v) in &self.headers {
            buf.push_str(&format!("{k}: {v}\r\n"));
        }
        buf
    }
}

impl From<&str> for Method {
    fn from(value: &str) -> Self {
        use Method::*;
        match value {
            "CONNECT" => CONNECT,
            "DELETE" => DELETE,
            "GET" => GET,
            "HEAD" => HEAD,
            "OPTIONS" => OPTIONS,
            "PATCH" => PATCH,
            "POST" => POST,
            "PUT" => PUT,
            "TRACE" => TRACE,
            _ => UNKNOWN,
        }
    }
}
