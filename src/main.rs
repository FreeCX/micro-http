use std::collections::HashMap;

pub mod app;
pub mod file;
pub mod http;
pub mod json;
pub mod random;
pub mod read;
pub mod status;

fn index(_request: http::HttpData) -> http::HttpData {
    file::response("./site/index.html")
}

fn file(request: http::HttpData) -> http::HttpData {
    file::response(&format!("./site{}", request.url))
}

fn api(request: http::HttpData) -> http::HttpData {
    let data = json::deserialize(&request.content.unwrap());
    let min_value: i32 = data.get("min").unwrap().parse().unwrap();
    let max_value: i32 = data.get("max").unwrap().parse().unwrap();

    let mut random = random::Random::new();
    let result = random.in_range(min_value, max_value);
    let mut data = HashMap::new();
    data.insert("result".to_string(), result.to_string());

    json::serialize(data)
}

fn main() {
    let mut app = app::App::new();
    app.bind(http::HttpMethod::GET, "/index", &index);
    app.bind(http::HttpMethod::GET, "/static/", &file);
    app.bind(http::HttpMethod::POST, "/api/", &api);
    app.run("127.0.0.1:8000");
}
