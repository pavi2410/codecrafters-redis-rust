use crate::resp::Resp;
use crate::kvstore::{KvStore, KvStatus};

pub struct Redis {
  kvstore: KvStore,
}

pub enum Command {
  Ping,
  Echo(String),
  Set(String, String, Option<u64>),
  Get(String),
}

pub enum Response {
  Pong,
  Echo(String),
  Ok,
  Value(String),
  Error(String),
  Null,
}

impl From<Resp> for Command {
    fn from(value: Resp) -> Self {
        match value {
            Resp::Array(a) => {
                match a[0] {
                    Resp::SimpleString(ref s) | Resp::BulkString(Some(ref s)) => {
                        match s.as_ref() {
                            "PING" | "ping" => {
                                Command::Ping
                            }
                            "ECHO" | "echo" => {
                                match a[1].clone() {
                                    Resp::SimpleString(s) | Resp::BulkString(Some(s)) => {
                                        Command::Echo(s)
                                    }
                                    _ => {
                                        Command::Echo("".to_string())
                                    }
                                }
                            }
                            "SET" | "set" => {
                                match a[1].clone() {
                                    Resp::SimpleString(key) | Resp::BulkString(Some(key)) => {
                                        match a[2].clone() {
                                            Resp::SimpleString(value) | Resp::BulkString(Some(value)) => {
                                                if a.len() == 3 {
                                                    Command::Set(key, value, None)
                                                } else {
                                                    match a[4].clone() {
                                                        Resp::BulkString(Some(expiry)) => {
                                                            Command::Set(key, value, expiry.parse::<u64>().ok())
                                                        }
                                                        _ => {
                                                            Command::Set(key, value, None)
                                                        }
                                                    }
                                                }
                                            }
                                            _ => {
                                                Command::Set(key, "".to_string(), None)
                                            }
                                        }
                                    }
                                    _ => {
                                        Command::Set("".to_string(), "".to_string(), None)
                                    }
                                }
                            }
                            "GET" | "get" => {
                                match a[1].clone() {
                                    Resp::SimpleString(key) | Resp::BulkString(Some(key)) => {
                                        Command::Get(key)
                                    }
                                    _ => {
                                        Command::Get("".to_string())
                                    }
                                }
                            }
                            _ => {
                                Command::Ping
                            }
                        }
                    }
                    _ => {
                        Command::Ping
                    }
                }
            }
            _ => {
                Command::Ping
            }
        }
    }
}

impl Into<Resp> for Response {
    fn into(self) -> Resp {
        match self {
            Response::Pong => {
                Resp::SimpleString("PONG".to_string())
            }
            Response::Echo(s) => {
                Resp::BulkString(Some(s))
            }
            Response::Ok => {
                Resp::SimpleString("OK".to_string())
            }
            Response::Value(s) => {
                Resp::BulkString(Some(s))
            }
            Response::Error(s) => {
                Resp::Error(s)
            }
            Response::Null => {
                Resp::BulkString(None)
            }
        }
    }
}

impl Redis {
  pub fn init() -> Redis {
    Redis {
      kvstore: KvStore::new(),
    }
  }

  pub fn handle_command(&mut self, input: Command) -> Response {
    match input {
      Command::Ping => {
        Response::Pong
      }
      Command::Echo(s) => {
        Response::Echo(s)
      }
      Command::Set(key, value, expiry) => {
        self.kvstore.set(key, value, expiry);
        Response::Ok
      }
      Command::Get(key) => {
        match self.kvstore.get(key) {
          KvStatus::Found(value) => {
            Response::Value(value)
          }
          KvStatus::KeyNotFound => {
            Response::Error("Key not found".to_string())
          }
          KvStatus::KeyExpired => {
            Response::Null
          }
        }
      }
    }
  }
}