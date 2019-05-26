use std::thread;
use std::time::Duration;
use std::fs;
use std::io::prelude::*;
use std::net::TcpStream;
use std::net::TcpListener;

use hello::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:60000").unwrap();
    let pool = ThreadPool::new(4);

    let mut count: u16 = 0;

    for stream in listener.incoming() {
        count = count + 1;
        println!("{}", count);

        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        println!("200 as usual");
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else if buffer.starts_with(sleep) {
        println!("200 lazy");
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();

    let response = format!("{}{}", status_line, contents);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}