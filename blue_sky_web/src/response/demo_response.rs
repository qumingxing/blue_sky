use derive_builder::Builder;
use serde::Serialize;
use serde_json::Value;

#[derive(Builder, Serialize)]
pub struct DemoResponse {
    message: String
}

#[derive(Builder, Serialize)]
pub struct LoginResponse {
    message: String
}