use crate::router::{Response};
use crate::server::HttpRequest;
use blue_sky_macro::route;

pub struct MyHandler;

#[route("GET", "/abc")]
fn handle_abc(request: &HttpRequest) -> Response {
    Response {
        status_code: 200,
        body: "Hello from /abc".to_string(),
    }
}
