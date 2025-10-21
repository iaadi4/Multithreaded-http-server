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
    let _http_request: Vec<_> = buf_reader // collects the lines of request send by browser
        .lines() // return iterators after spliting data when it sees newline
        .map(|result| result.unwrap()) // unwraps result, error if data it not valid utf-8
        .take_while(|line| !line.is_empty()) // iterates over unwrapped result till line is empty
        .collect(); // makes all request store in vector

    let status_line = "HTTP/1.1 200 OK";
    let contents = fs::read_to_string("public/hello.html").unwrap();
    let contents_length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {contents_length}\r\n\r\n{contents}");
    
    stream.write_all(response.as_bytes()).unwrap();
}