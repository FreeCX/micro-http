use crate::http;
use crate::status;
use std::io::Write;
use std::net::{TcpListener, TcpStream};

type RouteFunc = fn(http::HttpData) -> http::HttpData;

struct Route {
    url: String,
    method: http::HttpMethod,
    func: RouteFunc,
}

pub struct App {
    host: String,
    port: u16,
    routes: Vec<Route>,
}

impl App {
    pub fn new(host: &str, port: u16) -> App {
        App { routes: Vec::new(), host: host.to_string(), port }
    }

    pub fn bind(&mut self, method: http::HttpMethod, url: &str, func: RouteFunc) {
        self.routes.push(Route { url: url.to_string(), method, func })
    }

    fn route(&self, url: &str, method: http::HttpMethod) -> Option<&Route> {
        fn sublength(text: &str, subtext: &str) -> usize {
            text.chars().zip(subtext.chars()).take_while(|(a, b)| a == b).count()
        }

        let mut founded = None;
        let mut max_len = 0;

        for route in &self.routes {
            if route.method == method && url.starts_with(&route.url) {
                let curr_len = sublength(&route.url, url);
                if curr_len > max_len {
                    founded = Some(route);
                    max_len = curr_len;
                }
            }
        }

        founded
    }

    fn handle_client(&self, mut stream: TcpStream) -> Option<()> {
        let mut request = http::HttpData::new();
        request.addr = stream.peer_addr().ok();
        request.parse(&mut stream)?;

        if let Some(addr) = request.addr {
            println!(">>> incoming connection from {}:{}", addr.ip(), addr.port());
        }

        println!(">>> {:?} {}\n{}", request.method, request.url, request.render_headers());

        let mut response = self
            .route(&request.url, request.method)
            .map(|r| (r.func)(request))
            .unwrap_or(http::HttpData::from_status(status::StatusCode::NotFound));

        // add server info
        response.add_header("host", format!("{}:{}", self.host, self.port));
        response.add_header("server", "micro-http/0.1");

        println!("<<< HTTP/1.1 {}\n{}", response.status_code, response.render_headers());
        write!(stream, "HTTP/1.1 {}\r\n{}\r\n", response.status_code, response.render_headers()).ok()?;
        if let Some(content) = response.content {
            stream.write_all(content.as_slice()).ok()?;
        }

        Some(())
    }

    pub fn run(&self) -> Option<()> {
        let addr = format!("{}:{}", self.host, self.port);

        let listener = TcpListener::bind(&addr).ok()?;
        println!(">>> run server @ {addr}");

        for stream in listener.incoming().flatten() {
            self.handle_client(stream)?;
        }

        Some(())
    }
}
