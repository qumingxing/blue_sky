use crate::response::demo_response::{DemoResponseBuilder, LoginResponse, LoginResponseBuilder};
use crate::router::Response;
use crate::server::HttpRequest;
use blue_sky_macro::route;
use blue_sky_service::services::login_service::LoginService;
use std::io::{Error, ErrorKind};

#[route("POST", "/login")]
fn handle_login(request: &HttpRequest) -> Response {
    let request_value = &request.request_value;

    if let Some(p) = request_value {
        let user_name = p.get_string("user_name").unwrap();
        let password = p.get_string("password").unwrap();
        let login_service = LoginService::new(user_name, password);
        let res = login_service.login();
        let message = if res { "登录成功" } else { "登录失败" };

        let response = LoginResponseBuilder::default()
            .message(message.to_string())
            .build()
            .unwrap();
        Response {
            status_code: 200,
            data: Some(serde_json::to_value(response).unwrap()),
        }
    } else {
        let response = LoginResponseBuilder::default()
            .message("登录失败".to_string())
            .build()
            .unwrap();
        Response {
            status_code: 400,
            data: Some(serde_json::to_value(response).unwrap()),
        }
    }
}
