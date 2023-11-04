use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::thread;

use resp::Resp;

mod resp;

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
    let mut buf = [0; 1024];

    loop {
        stream.read(&mut buf).unwrap();

        let s = String::from_utf8_lossy(&buf[..]).trim_end().to_owned();

        let c = Resp::decode(&s).unwrap();

        match c {
            Resp::Array(a) => {
                match a[0] {
                    Resp::SimpleString(ref s) => {
                        match s.as_ref() {
                            "PING" | "ping" => {
                                stream.write(Resp::SimpleString("PONG".to_string()).encode().as_slice()).unwrap();
                            }
                            "ECHO" | "echo" => {
                                stream.write(a[1].encode().as_slice()).unwrap();
                            }
                            k => {
                                println!("unknown command: {}", k);
                                stream.write(Resp::Error("unknown command".to_string()).encode().as_slice()).unwrap();
                            }
                        }
                    }
                    _ => {
                        stream.write(Resp::Error("unknown command".to_string()).encode().as_slice()).unwrap();
                    }
                }
            },
            _ => {
                stream.write(Resp::Error("unknown command".to_string()).encode().as_slice()).unwrap();
            }
        }
    }
}
