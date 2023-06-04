pub mod app;
pub mod file;
pub mod http;
pub mod json;
pub mod random;
pub mod read;
pub mod status;

use app::App;
use http::{HttpData, HttpMethod};
use json::SimpleJson;
use random::Random;
use status::StatusCode;

fn api(request: HttpData) -> HttpData {
    fn process(request: HttpData) -> Option<HttpData> {
        let data = request.content.and_then(|c| json::deserialize(&c))?;
        let minv = data.get("min").cloned().and_then(|d| d.parse().ok())?;
        let maxv = data.get("max").cloned().and_then(|d| d.parse().ok())?;

        let result = Random::new().in_range(minv, maxv);

        let mut data = SimpleJson::new();
        data.insert("result".to_string(), result.to_string());

        Some(json::serialize(data))
    }

    match process(request) {
        Some(r) => r,
        None => HttpData::from_status(StatusCode::BadRequest),
    }
}

fn main() {
    let mut app = App::new("127.0.0.1", 8000);
    app.bind("/", HttpMethod::GET, |_| file::response("./site/index.html"));
    app.bind("/static/", HttpMethod::GET, |r| file::response(&format!("./site{}", r.url)));
    app.bind("/api/", HttpMethod::POST, api);
    app.run();
}
