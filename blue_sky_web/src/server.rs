use crate::request_mapping::register_request_mapping;
use crate::router::{Response, Router};
use lazy_static::lazy_static;
use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::mpsc::Sender;
use std::sync::{mpsc, Arc, Mutex};
use std::{env, fs};
use threadpool::ThreadPool;

lazy_static! {
    static ref POOL: Mutex<ThreadPool> = Mutex::new(ThreadPool::new(4));
    pub static ref INSTANCE: Mutex<Router> = Mutex::new(Router::new());
}

pub fn start_server() {
    log::info!("Starting register request mapping!");
    register_request_mapping();
    log::info!("Register success!");
    let listener = TcpListener::bind("0.0.0.0:8080").unwrap();
    println!("Server running on 0.0.0.0:8080");
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
        println!("Connection established!");
    }
}
fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();
    let http_request = parse_protocol(&buffer);
    let (tx, rx) = mpsc::channel();
    handle_request(tx, Arc::new(Mutex::new(http_request)));
    let response = rx.recv().unwrap();
    if let Some(res) = response {
        let res_json = serde_json::to_string(&res);
        match res_json {
            Ok(res_json) => {
                let response = format!(
                    "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{}",
                    res_json.len(),
                    res_json
                );

                stream.write(response.as_bytes()).unwrap();
                stream.flush().unwrap();
            }
            Err(e) => {
                eprintln!("Response error: {:?}", e);
            }
        }
    } else {
        let contents = String::from(
            r#"<html>
                <head><title>404 Not Found</title></head>
                <body><h1>404 Not Found</h1></body>
                </html>"#,
        );
        let response = format!(
            "HTTP/1.1 404 Not Found\r\nContent-Length: {}\r\nContent-Type: text/html\r\n\r\n{}",
            contents.len(),
            contents
        );

        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }
}
fn handle_request(
    sender: Sender<Option<Response>>,
    http_request: Arc<Mutex<HttpRequest>>,
) -> Option<Response> {
    let mut response: Option<Response> = None;
    POOL.lock().unwrap().execute(move || {
        let router = &mut INSTANCE.lock().unwrap();

        let request = &http_request.lock().unwrap();
        let route_fn = router.get_route(
            request.method.to_string().as_str(),
            request.context_path.as_str(),
        );
        if let Some(route) = route_fn {
            sender.send(Some(route(request))).unwrap();
        } else {
            sender.send(None).unwrap();
        }
    });
    response
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
enum Method {
    GET,
    POST,
    DELETE,
}
impl Display for Method {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Method::GET => write!(f, "GET"),
            Method::POST => write!(f, "POST"),
            Method::DELETE => write!(f, "DELETE"),
        }
    }
}
impl Default for Method {
    fn default() -> Self {
        Method::GET
    }
}
#[derive(Debug, PartialEq, Eq)]
pub struct HttpRequest {
    context_path: String,
    method: Method,
    pub headers: HashMap<String, String>,
    pub request_value: Option<RequestValue>,
    pub body: Option<Value>,
}
#[derive(Debug, PartialEq, Eq)]
pub struct RequestValue {
    request_params: HashMap<String, String>,
}
impl RequestValue {
    fn new(request_params: HashMap<String, String>) -> RequestValue {
        RequestValue { request_params }
    }
    pub fn get_int(&self, key: &str) -> Option<i32> {
        match self.request_params.get(key) {
            Some(value) => Some(value.parse::<i32>().unwrap()),
            None => None,
        }
    }

    pub fn get_string(&self, key: &str) -> Option<String> {
        match self.request_params.get(key) {
            Some(value) => Some(value.to_string()),
            None => None,
        }
    }
    pub fn get_float(&self, key: &str) -> Option<f32> {
        match self.request_params.get(key) {
            Some(value) => Some(value.parse::<f32>().unwrap()),
            None => None,
        }
    }
    pub fn get_double(self, key: &str) -> Option<f64> {
        match self.request_params.get(key) {
            Some(value) => Some(value.parse::<f64>().unwrap()),
            None => None,
        }
    }
    pub fn get_bool(&self, key: &str) -> Option<bool> {
        match self.request_params.get(key) {
            Some(value) => Some(value.parse::<bool>().unwrap()),
            None => None,
        }
    }
}
fn parse_protocol(buffer: &[u8; 1024]) -> HttpRequest {
    let original_content = String::from_utf8_lossy(&buffer[..]);
    let raw_lines = original_content.split("\r\n").collect::<Vec<&str>>();
    let request_method_params = raw_lines[0].split(" ").collect::<Vec<&str>>();
    let method = match request_method_params[0] {
        "GET" => Method::GET,
        "POST" => Method::POST,
        "DELETE" => Method::DELETE,
        _ => Method::default(),
    };

    let context_path_params = request_method_params[1];
    let mut request_params: HashMap<String, String> = HashMap::new();
    let context_path = if let Some(end_pos) = context_path_params.find("?") {
        let cut_params = request_method_params[1][end_pos + 1..].to_string();
        let params = cut_params.split("&").collect::<Vec<&str>>();
        request_params = parse_params(&params);
        request_method_params[1][0..end_pos].to_string()
    } else if let None = context_path_params.find("?") {
        context_path_params.to_string()
    } else {
        "/".to_string()
    };

    let request_value = RequestValue::new(request_params);

    let mut headers = HashMap::new();
    let mut body: Option<Value> = None;
    let re = Regex::new(r#"\{.*?\}|\[.*?\]"#).unwrap();
    for (index, value) in raw_lines.iter().enumerate().skip(1) {
        let request_entry = value.split(": ").collect::<Vec<&str>>();
        if request_entry.len() == 2 {
            let key = request_entry[0].trim().to_string();
            let value = request_entry[1].trim().to_string();
            headers.insert(key, value);
        } else {
            if headers.contains_key("Content-Type")
                && headers
                    .get("Content-Type")
                    .unwrap()
                    .contains("application/json")
            {
                if let Some(captures) = re.captures(value) {
                    if let Some(json) = captures.get(0) {
                        body = Some(json.as_str().into());
                    }
                }
            }
            if headers.contains_key("Content-Type")
                && headers
                    .get("Content-Type")
                    .unwrap()
                    .contains("application/x-www-form-urlencoded")
            {
                if value.contains("&") || value.contains("=") {
                    let params = value.split("&").collect::<Vec<&str>>();
                    request_params = parse_params(&params);
                }
            }
        }
    }

    HttpRequest {
        method,
        context_path,
        headers,
        request_value: Some(request_value),
        body,
    }
}

fn parse_params(params: &Vec<&str>) -> HashMap<String, String> {
    let mut request_params = HashMap::new();
    for param in params {
        let p = param.split("=").collect::<Vec<&str>>();
        if p.len() % 2 == 0 {
            request_params.insert(p[0].to_string(), p[1].to_string());
        }
    }
    request_params
}
