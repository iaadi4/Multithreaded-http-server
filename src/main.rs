use std::fs;
use std::io::BufReader;
use std::net::TcpListener;
use std::net::TcpStream;
use std::io::prelude::*;


fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    let (status_line, content_path) = if request_line == "GET / HTTP/1.1" {
        ("HTTP/1.1 200 OK", "public/hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "public/404.html")
    };

    let contents = fs::read_to_string(content_path).unwrap();
    let contents_length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {contents_length}\r\n\r\n{contents}");
    
    stream.write_all(response.as_bytes()).unwrap();
}