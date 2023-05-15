use crate::http;
use crate::status;
use std::io::Write;
use std::net::{TcpListener, TcpStream};

type RouteFunc = dyn Fn(http::HttpData) -> http::HttpData + 'static;

struct Route {
    url: String,
    method: http::HttpMethod,
    func: Box<RouteFunc>,
}

pub struct App {
    routes: Vec<Route>,
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

impl App {
    pub fn new() -> App {
        App { routes: Vec::new() }
    }

    pub fn bind<F>(&mut self, method: http::HttpMethod, url: &str, func: F)
    where
        F: Fn(http::HttpData) -> http::HttpData + 'static,
    {
        self.routes.push(Route { url: url.to_string(), method, func: Box::new(func) })
    }

    // TODO: реализовать Keep-Alive соединение
    fn handle_client(&self, mut stream: TcpStream) {
        let mut request = http::HttpData::new();
        request.addr = stream.local_addr().ok();
        request.parse(&mut stream);

        println!(">>> {:?} {}\n{}", request.method.unwrap(), request.url, request.render_headers());

        let response = match self
            .routes
            .iter()
            .filter(|x| request.url.starts_with(&x.url) && x.method == request.method.unwrap())
            .take(1)
            .next()
        {
            Some(route) => (route.func)(request),
            None => {
                let mut response = http::HttpData::new();
                response.status_code = status::StatusCode::NotFound;
                response
            }
        };

        println!("<<< {}\n{}", response.status_code, response.render_headers());

        let _ = write!(stream, "HTTP/1.1 {}\r\n{}\r\n", response.status_code, response.render_headers());
        if let Some(content) = response.content {
            let _ = stream.write_all(content.as_slice());
        }
    }

    pub fn run(&self, host: &str) {
        let listener = TcpListener::bind(host).unwrap();
        for stream in listener.incoming() {
            self.handle_client(stream.unwrap());
        }
    }
}
