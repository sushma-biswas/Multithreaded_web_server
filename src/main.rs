// starting with a single-threaded web server

use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::fs;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    // b stands for byte string
    let get = b"GET / HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        // an OK response for correct request
        ("HTTPS/1.1 200 OK\r\n\r\n", "hello.html")
    } else {
        // a customized error response for incorrect requests
        ("HTTPS/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };
    
    let content = fs::read_to_string(filename).unwrap();
    let response = format!("{}{}", status_line, content);
    
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}