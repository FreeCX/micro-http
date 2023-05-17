use crate::http;
use crate::status;
use std::io::Write;
use std::net::{TcpListener, TcpStream};

type RouteFunc = dyn Fn(http::HttpData) -> http::HttpData;

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

    pub fn bind(&mut self, method: http::HttpMethod, url: &str, func: &'static RouteFunc) {
        self.routes.push(Route { url: url.to_string(), method, func: Box::new(func) })
    }

    fn route(&self, url: &str, method: http::HttpMethod) -> Option<&Route> {
        let sublength =
            |text: &str, subtext: &str| text.chars().zip(subtext.chars()).take_while(|(a, b)| a == b).count();
        let mut founded = None;
        let mut max_len = 0;

        for route in &self.routes {
            if url.starts_with(&route.url) && route.method == method {
                let curr_len = sublength(&route.url, url);
                if curr_len > max_len {
                    founded = Some(route);
                    max_len = curr_len;
                }
            }
        }

        founded
    }

    // TODO: реализовать Keep-Alive соединение
    fn handle_client(&self, mut stream: TcpStream) {
        let mut request = http::HttpData::new();
        request.addr = stream.local_addr().ok();
        request.parse(&mut stream);

        println!(">>> {:?} {}\n{}", request.method.unwrap(), request.url, request.render_headers());

        let response = match self.route(&request.url, request.method.unwrap()) {
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
