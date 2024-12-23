use crate::router::{Response, Router};
use crate::users::user_action::MyHandler;
use lazy_static::lazy_static;
use regex::Regex;
use serde::{Deserialize, Serialize};
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
    static ref INSTANCE: Mutex<Router> = Mutex::new(Router::new());
}

pub fn start_server() {
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
    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
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
        MyHandler::register_route(router);
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
    headers: HashMap<String, String>,
    request_value: RequestValue,
}
#[derive(Debug, PartialEq, Eq)]
struct RequestValue {
    request_params: HashMap<String, String>,
}
impl RequestValue {
    fn new(request_params: HashMap<String, String>) -> RequestValue {
        RequestValue { request_params }
    }
    fn get_int(&self, key: &str) -> Option<i32> {
        match self.request_params.get(key) {
            Some(value) => Some(value.parse::<i32>().unwrap()),
            None => None,
        }
    }

    fn get_string(&self, key: &str) -> Option<String> {
        match self.request_params.get(key) {
            Some(value) => Some(value.to_string()),
            None => None,
        }
    }
    fn get_float(&self, key: &str) -> Option<f32> {
        match self.request_params.get(key) {
            Some(value) => Some(value.parse::<f32>().unwrap()),
            None => None,
        }
    }
    fn get_double(self, key: &str) -> Option<f64> {
        match self.request_params.get(key) {
            Some(value) => Some(value.parse::<f64>().unwrap()),
            None => None,
        }
    }
    fn get_bool(&self, key: &str) -> Option<bool> {
        match self.request_params.get(key) {
            Some(value) => Some(value.parse::<bool>().unwrap()),
            None => None,
        }
    }
}
fn parse_protocol(buffer: &[u8; 1024]) -> HttpRequest {
    let original_content = String::from_utf8_lossy(&buffer[..]);
    let raw_lines = original_content.split("\n").collect::<Vec<&str>>();
    let request_method_params = raw_lines[0].split(" ").collect::<Vec<&str>>();
    let method = match request_method_params[0] {
        "GET" => Method::GET,
        "POST" => Method::POST,
        "DELETE" => Method::DELETE,
        _ => Method::default(),
    };

    let context_path_params = request_method_params[1];
    let context_path = if let Some(end_pos) = context_path_params.find("?") {
        request_method_params[1][0..end_pos].to_string()
    } else {
        "/".to_string()
    };
    let regex = Regex::new(r"[?/]").unwrap();
    let params = regex.replace_all(context_path_params, "");
    let mut params = params.split("&").collect::<Vec<&str>>();

    let mut request_params = HashMap::new();
    for param in params {
        let p = param.split("=").collect::<Vec<&str>>();
        request_params.insert(p[0].to_string(), p[1].to_string());
    }

    let request_value = RequestValue::new(request_params);

    let mut headers = HashMap::new();
    for (index, value) in raw_lines.iter().enumerate().skip(1) {
        let request_entry = value.split(": ").collect::<Vec<&str>>();
        if request_entry.len() == 2 {
            let key = request_entry[0].trim().to_string();
            let value = request_entry[1].trim().to_string();
            headers.insert(key, value);
        }
    }

    HttpRequest {
        method,
        context_path,
        headers,
        request_value,
    }
}
