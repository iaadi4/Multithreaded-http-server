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
    let http_request: Vec<_> = buf_reader // collects the lines of request send by browser
        .lines() // return iterators after spliting data when it sees newline
        .map(|result| result.unwrap()) // unwraps result, error if data it not valid utf-8
        .take_while(|line| !line.is_empty()) // iterates over unwrapped result till line is empty
        .collect(); // makes all request store in vector

    let response = "HTTP/1.1 200 OK\r\n\r\n";

    stream.write_all(response.as_bytes()).unwrap();
}