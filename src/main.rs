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

// TODO: optimize
fn api(request: http::HttpData) -> http::HttpData {
    let data = match request.content.and_then(|c| json::deserialize(&c)) {
        Some(dict) => dict,
        None => {
            let mut response = http::HttpData::new();
            response.status_code = status::StatusCode::BadRequest;
            return response;
        }
    };

    let min_value = data.get("min").cloned().and_then(|d| d.parse().ok());
    let max_value = data.get("max").cloned().and_then(|d| d.parse().ok());

    match (min_value, max_value) {
        (Some(minv), Some(maxv)) => {
            let mut random = random::Random::new();
            let result = random.in_range(minv, maxv);

            let mut data = HashMap::new();
            data.insert("result".to_string(), result.to_string());

            json::serialize(data)
        }
        _ => {
            let mut response = http::HttpData::new();
            response.status_code = status::StatusCode::BadRequest;
            response
        }
    }
}

fn main() {
    let mut app = app::App::new();
    app.bind(http::HttpMethod::GET, "/", &index);
    app.bind(http::HttpMethod::GET, "/static/", &file);
    app.bind(http::HttpMethod::POST, "/api/", &api);
    app.run("127.0.0.1:8000");
}
