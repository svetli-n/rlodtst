use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::{fs, thread};
use std::time::Duration;

use rustwebsrv::{ThreadPool};

fn main() {
    let pool = ThreadPool::new(4);
    let listener = TcpListener::bind("localhost:8787").unwrap();
    for stream in listener.incoming().take(2) {
        let stream = stream.unwrap();
        pool.execute(|| handle_stream(stream));
    }
}

fn handle_stream(mut stream: TcpStream) {
    let mut buf = [0; 1024];
    stream.read(&mut buf).unwrap();

    println!("{:?}", String::from_utf8_lossy(&buf[..]));

    let hello = "GET /hello".as_bytes();
    let sleep = "GET /sleep".as_bytes();

    let (status_line, filename) = if buf.starts_with(hello) {
        ("HTTP/1.1. 200 OK", "src/hello.html")
    } else if buf.starts_with(sleep) {
        thread::sleep(Duration::new(5, 0));
        ("HTTP/1.1. 200 OK", "src/hello.html")
    }
    else {
        ("HTTP/1.1 404 NOT FOUND", "src/404.html")
    };
    let contents = fs::read_to_string(filename).unwrap();
    let resp = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );
    stream.write(resp.as_bytes()).unwrap();
    stream.flush().unwrap();


}

