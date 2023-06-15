extern crate micro_http;

use micro_http::app::App;
use micro_http::file;
use micro_http::http::{Data, Method};
use micro_http::json::{self, SimpleJson};
use micro_http::random::Random;
use micro_http::status::StatusCode;

fn api(request: Data) -> Data {
    fn process(request: Data) -> Option<Data> {
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
        None => Data::from_status(StatusCode::BadRequest),
    }
}

fn main() {
    let mut app = App::new("127.0.0.1", 8000);
    app.bind("/", Method::GET, |_| file::response("./site/index.html"));
    app.bind("/static/", Method::GET, |r| file::response(&format!("./site{}", r.url)));
    app.bind("/api/", Method::POST, api);
    app.run(4);
}
