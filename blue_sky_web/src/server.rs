use std::{env, fs};
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

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

    let get = b"GET / HTTP/1.1\r\n";

    if buffer.starts_with(get) {
        let dir = env::current_dir().unwrap();
        let html = format!("{}{}", dir.to_str().unwrap(), "/blue_sky_web/hello.html");
        let contents = fs::read_to_string(html).unwrap();

        let response = format!(
            "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
            contents.len(),
            contents
        );

        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    } else {
        // other requests
    }
}