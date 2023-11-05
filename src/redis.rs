use crate::resp::Resp;
use crate::kvstore::KvStore;

pub struct Redis {
  kvstore: KvStore,
}

pub enum Command {
  Ping,
  Echo(String),
  Set(String, String),
  Get(String),
}

pub enum Response {
  Pong,
  Echo(String),
  Ok,
  Value(String),
  Error(String),
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
                                                Command::Set(key, value)
                                            }
                                            _ => {
                                                Command::Set(key, "".to_string())
                                            }
                                        }
                                    }
                                    _ => {
                                        Command::Set("".to_string(), "".to_string())
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
      Command::Set(key, value) => {
       self.kvstore.set(key, value);
        Response::Ok
      }
      Command::Get(key) => {
        match self.kvstore.get(key) {
          Some(value) => {
            Response::Value(value)
          }
          None => {
            Response::Error("Key not found".to_string())
          }
        }
      }
    }
  }
}