use crate::router::{Response};
use crate::server::HttpRequest;
use blue_sky_macro::route;

#[route("GET", "/demo")]
fn handle_demo(request: &HttpRequest) -> Response {
    Response {
        status_code: 200,
        body: "Hello from /demo".to_string(),
    }
}

#[route("GET", "/demo_other")]
fn handle_demo_other(request: &HttpRequest) -> Response {
    Response {
        status_code: 200,
        body: "Hello from /demo_other".to_string(),
    }
}