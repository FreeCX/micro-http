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
    fn process(request: http::HttpData) -> Option<http::HttpData> {
        let data = request.content.and_then(|c| json::deserialize(&c))?;
        let minv = data.get("min").cloned().and_then(|d| d.parse().ok())?;
        let maxv = data.get("max").cloned().and_then(|d| d.parse().ok())?;

        let result = random::Random::new().in_range(minv, maxv);

        let mut data = HashMap::new();
        data.insert("result".to_string(), result.to_string());

        Some(json::serialize(data))
    }

    match process(request) {
        Some(r) => r,
        None => http::HttpData::from_status(status::StatusCode::BadRequest),
    }
}

fn main() {
    let mut app = app::App::new("127.0.0.1", 8000);
    app.bind(http::HttpMethod::GET, "/", index);
    app.bind(http::HttpMethod::GET, "/static/", file);
    app.bind(http::HttpMethod::POST, "/api/", api);
    app.run();
}
