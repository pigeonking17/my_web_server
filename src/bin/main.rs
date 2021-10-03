use lazy_static::lazy_static;
use regex::Regex;
use std::thread;
use std::time::Duration;
use std::str;
use std::fs;
use std::io::prelude::*;
use std::net::TcpStream;
use std::net::TcpListener;

use hello::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        
        pool.execute(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];

    stream.read(&mut buffer).unwrap();

    lazy_static! {
        static ref GET_RE: Regex = Regex::new("^GET (?P<request_file>/.*) HTTP").unwrap();
    }

    let caps = GET_RE.captures(str::from_utf8(&buffer).unwrap()).unwrap();

    let request_file = &caps["request_file"];

    let response: String;

    match request_file {
        "/" => {
            let contents = fs::read_to_string("hello.html").unwrap();
            response = format!("HTTP/1.1 200 OK\r\n\r\n{}", contents);
        }
        "/sleep" => {
            thread::sleep(Duration::from_secs(5));
            let contents = fs::read_to_string("hello.html").unwrap();
            response = format!("HTTP/1.1 200 OK\r\n\r\n{}", contents);
        }
        _ => {
            let contents = fs::read_to_string("404.html").unwrap();
            response = format!("HTTP/1.1 404 NOT FOUND\r\n\r\n{}", contents);
        }
    }

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
