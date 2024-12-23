use crate::router::{Response};
use crate::server::HttpRequest;
use blue_sky_macro::route;

pub struct MyHandler;

#[route("GET", "/demo")]
fn handle_demo(request: &HttpRequest) -> Response {
    Response {
        status_code: 200,
        body: "Hello from /demo".to_string(),
    }
}
