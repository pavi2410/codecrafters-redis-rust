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
        let bytes_read = stream.read(&mut buf).unwrap();

        let s = String::from_utf8_lossy(&buf[..bytes_read]);

        println!("received: {:?}", s);

        let c = Resp::decode(s.to_string()).unwrap();

        let r = handle_redis_commands(c);

        let bytes = r.encode();

        stream.write(&bytes).unwrap();
    }
}

fn handle_redis_commands(input: Resp) -> Resp {
    match input {
        Resp::Array(a) => {
            println!("command: {:#?}", a);
            match a[0] {
                Resp::SimpleString(ref s) | Resp::BulkString(Some(ref s)) => {
                    match s.as_ref() {
                        "PING" | "ping" => {
                            Resp::SimpleString("PONG".to_string())
                        }
                        "ECHO" | "echo" => {
                            a[1].clone()
                        }
                        _ => {
                            Resp::Error("unknown command".to_string())
                        }
                    }
                }
                _ => {
                    Resp::Error("unknown command".to_string())
                }
            }
        },
        _ => {
            Resp::Error("unknown command".to_string())
        }
    }
}