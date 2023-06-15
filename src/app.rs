use std::io::Write;
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time::{Duration, SystemTime};

use crate::error::FrameworkError;
use crate::http::{Data, Method};
use crate::status::StatusCode;

type RouteFunc = fn(Data) -> Data;

#[derive(Clone)]
struct Route {
    url: String,
    method: Method,
    func: RouteFunc,
}

#[derive(Clone)]
pub struct App {
    host: String,
    port: u16,
    routes: Vec<Route>,
}

impl App {
    pub fn new(host: &str, port: u16) -> App {
        App { routes: Vec::new(), host: host.to_string(), port }
    }

    pub fn bind(&mut self, url: &str, method: Method, func: RouteFunc) {
        self.routes.push(Route { url: url.to_string(), method, func })
    }

    fn route(&self, url: &str, method: Method) -> Option<&Route> {
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

    fn handle_client(&self, mut stream: TcpStream) -> Result<(), FrameworkError> {
        if let Ok(addr) = stream.peer_addr() {
            println!(">>> incoming connection from {}:{}", addr.ip(), addr.port());
        }

        let connection_start = SystemTime::now();
        stream.set_read_timeout(Some(Duration::from_secs(5)))?;
        stream.set_write_timeout(Some(Duration::from_secs(5)))?;

        loop {
            let elapsed = connection_start.elapsed().unwrap_or(Duration::from_secs(5));
            if elapsed >= Duration::from_secs(5) {
                break;
            }

            let mut request = Data::new();
            request.addr = stream.peer_addr().ok();
            request.parse(&mut stream)?;

            let keep_alive = request.headers.get("connection").map(|t| t == "keep-alive").unwrap_or(false);

            println!(">>> {:?} {}\n{}", request.method, request.url, request.render_headers());

            let mut response = self
                .route(&request.url, request.method)
                .map(|r| (r.func)(request))
                .unwrap_or(Data::from_status(StatusCode::NotFound));

            // add server info
            response.add_header("host", format!("{}:{}", self.host, self.port));
            response.add_header("server", "micro-http/0.1");
            if keep_alive {
                response.add_header("connection", "keep-alive");
            }

            println!("<<< HTTP/1.1 {}\n{}", response.status_code, response.render_headers());
            write!(stream, "HTTP/1.1 {}\r\n{}\r\n", response.status_code, response.render_headers())?;
            if let Some(content) = response.content {
                stream.write_all(content.as_slice())?;
            }

            if !keep_alive {
                break;
            }
        }

        println!("--- end of connection ---");

        Ok(())
    }

    // TODO: использовать ограниченное число потоков
    pub fn run(&self, _threads: u16) -> Option<()> {
        let addr = format!("{}:{}", self.host, self.port);

        let listener = TcpListener::bind(&addr).ok()?;
        println!(">>> run server @ {addr}");

        for stream in listener.incoming().flatten() {
            let app_clone = self.clone();
            thread::spawn(move || {
                if let Some(err) = app_clone.handle_client(stream).err() {
                    println!("!!! thread was stopped: {err}");
                }
            });
        }

        Some(())
    }
}
