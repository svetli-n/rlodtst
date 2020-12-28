use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::{fs, thread};
use std::time::{Duration, Instant};
use std::sync::mpsc;

use rustwebsrv::{ThreadPool};
use std::sync::mpsc::Sender;
use std::convert::TryInto;

fn main() {
    let pool = ThreadPool::new(4);
    let listener = TcpListener::bind("localhost:8787").unwrap();
    for stream in listener.incoming() {
        let (sender, receiver) = mpsc::channel();
        let mut stream = stream.unwrap();

        let mut i = 0;
        let  requests = get_requests();
        for req in requests.clone() {
            let snder = mpsc::Sender::clone(&sender);
            pool.execute(move || execute(i, req, snder));
            i += 1;
        }

        let mut is_ok_cnt = 0;
        let mut durations = 0;

        loop {
            if i == 0 {
                break;
            }
            i -= 1;
            let result = receiver.recv().unwrap();
            if result.is_ok {
                is_ok_cnt += 1;
                durations += result.duration.as_millis();
            }
        }
        let rlen = requests.len() as f32;
        let durations = durations as f32;
        respond(&mut stream, is_ok_cnt, durations / rlen);
    }
}

fn respond(stream: &mut TcpStream, is_ok_cnt: i32, avg_duration: f32) {
    let contents = format!("OK, is_ok_cnt: {}, avg_duration in millis: {}", is_ok_cnt, avg_duration);
    let resp = format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
        contents.len(),
        contents
    );
    stream.write(resp.as_bytes());
    stream.flush();
}

fn execute(i: usize, url: String, sender: Sender<Resp>) {
    println!("exec {} {}", i, url);
    let start = Instant::now();
    let is_ok = reqwest::blocking::get(url.as_str()).is_ok();
    let duration = Instant::now() - start;
    sender.send(Resp {is_ok, duration});
}

fn get_requests() -> Vec<String> {
    let requests = fs::read_to_string("src/requests.txt").unwrap();
    let mut res = Vec::with_capacity(requests.len());
    for request in requests.lines() {
       res.push(request.to_string());
    }
    res
}

struct Resp {
    is_ok: bool,
    duration: Duration,
}

