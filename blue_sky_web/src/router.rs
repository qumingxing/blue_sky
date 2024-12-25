use crate::server::HttpRequest;
use serde::{Serialize, Serializer};
use std::collections::HashMap;
use serde_json::{Value};

pub trait RouteHandler {
    fn handle(&self, request: &HttpRequest) -> Response;
}
#[derive(Default,Serialize)]
pub struct Response {
    pub status_code: u16,
    pub data: Option<Value>,
}

pub struct Router {
    routers: HashMap<(String, String), fn(&HttpRequest) -> Response>,
}

impl Router {
    pub fn new() -> Self {
        Router {
            routers: HashMap::new(),
        }
    }
    pub fn add_route(&mut self, method: &str, path: &str, handler: fn(&HttpRequest) -> Response) {
        self.routers
            .insert((method.to_string(), path.to_string()), handler);
    }

    pub fn get_route(&self, method: &str, path: &str) -> Option<fn(&HttpRequest) -> Response> {
        if let Some(handler) = self.routers.get(&(method.to_string(), path.to_string())) {
            Some(*handler)
        } else {
            None
        }
    }
}
