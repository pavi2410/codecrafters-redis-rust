use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::thread;

use redis::Redis;
use resp::Resp;

mod resp;
mod redis;
mod kvstore;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();
    
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("accepted new connection");

                thread::spawn(|| {
                    handle_connection(stream);
                });
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut redis = Redis::init();

    let mut buf = [0; 1024];

    loop {
        let bytes_read = stream.read(&mut buf).unwrap();

        let s = String::from_utf8_lossy(&buf[..bytes_read]);

        println!("received: {:?}", s);

        let c: Resp = Resp::decode(&mut s.chars()).unwrap();

        let r: Resp = redis.handle_command(c.into()).into();

        let bytes = r.encode();

        stream.write(&bytes).unwrap();
    }
}
