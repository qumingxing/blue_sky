use crate::response::demo_response::DemoResponseBuilder;
use crate::router::Response;
use crate::server::HttpRequest;
use blue_sky_macro::route;

#[route("GET", "/demo")]
fn handle_demo(request: &HttpRequest) -> Response {
    let response = DemoResponseBuilder::default()
        .message(String::from("Hello"))
        .build()
        .unwrap();
    Response {
        status_code: 200,
        data: Some(serde_json::to_value(response).unwrap()),
    }
}

#[route("POST", "/demo_other")]
fn handle_demo_other(request: &HttpRequest) -> Response {
    Response {
        status_code: 200,
        ..Default::default()
    }
}
