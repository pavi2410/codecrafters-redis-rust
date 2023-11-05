#[derive(Debug, PartialEq, Clone)]
pub enum Resp {
    SimpleString(String),
    Error(String),
    Integer(i64),
    BulkString(Option<String>),
    Array(Vec<Resp>),
}

impl Resp {
  pub fn encode(&self) -> Vec<u8> {
    match self {
      Resp::SimpleString(s) => format!("+{}\r\n", s).into_bytes(),
      Resp::Error(s) => format!("-{}\r\n", s).into_bytes(),
      Resp::Integer(i) => format!(":{}\r\n", i).into_bytes(),
      Resp::BulkString(Some(s)) => format!("${}\r\n{}\r\n", s.len(), s).into_bytes(),
      Resp::BulkString(None) => "$-1\r\n".to_owned().into_bytes(),
      Resp::Array(a) => {
        let mut bytes = format!("*{}\r\n", a.len()).into_bytes();

        for e in a {
          bytes.extend(e.encode());
        }

        bytes
      }
    }
  }

  pub fn decode(s: String) -> Result<Resp, String> {
    let mut chars = s.chars();

    match chars.next() {
      Some('+') => {
        let mut s = String::new();

        loop {
          match chars.next() {
            Some('\r') => {
              match chars.next() {
                Some('\n') => {
                  return Ok(Resp::SimpleString(s));
                }
                _ => {
                  return Err("45. expected newline".to_string());
                }
              }
            }
            Some(c) => {
              s.push(c);
            }
            None => {
              return Err("53. expected newline".to_string());
            }
          }
        }
      }
      Some('-') => {
        let mut s = String::new();

        loop {
          match chars.next() {
            Some('\r') => {
              match chars.next() {
                Some('\n') => {
                  return Ok(Resp::Error(s));
                }
                _ => {
                  return Err("69. expected newline".to_string());
                }
              }
            }
            Some(c) => {
              s.push(c);
            }
            None => {
              return Err("77. expected newline".to_string());
            }
          }
        }
      }
      Some(':') => {
        let mut s = String::new();

        loop {
          match chars.next() {
            Some('\r') => {
              match chars.next() {
                Some('\n') => {
                  match s.parse::<i64>() {
                    Ok(i) => {
                      return Ok(Resp::Integer(i));
                    }
                    Err(_) => {
                      return Err("expected integer".to_string());
                    }
                  }
                }
                _ => {
                  return Err("100. expected newline".to_string());
                }
              }
            }
            Some(c) => {
              s.push(c);
            }
            None => {
              return Err("108. expected newline".to_string());
            }
          }
        }
      }
      Some('$') => {
        let mut s = String::new();

        loop {
          match chars.next() {
            Some('\r') => {
              match chars.next() {
                Some('\n') => {
                  match s.parse::<i64>() {
                    Ok(i) => {
                      if i == -1 {
                        return Ok(Resp::BulkString(None));
                      } else if i >= 0 {
                        let buf = chars.clone().take(i as usize).collect::<String>();

                        println!("buf: {:?}", buf);

                        match chars.next() {
                          Some('\r') => {
                            match chars.next() {
                              Some('\n') => {
                                return Ok(Resp::BulkString(Some(buf)));
                              }
                              _ => {
                                return Err("135. expected newline".to_string());
                              }
                            }
                          }
                          k => {
                            println!("k: {:?}", k);
                            return Err("140. expected newline".to_string());
                          }
                        }
                      } else {
                        return Err("invalid string length".to_string());
                      }
                    }
                    Err(_) => {
                      return Err("expected integer".to_string());
                    }
                  }
                }
                _ => {
                  return Err("153. expected newline".to_string());
                }
              }
            }
            Some(c) => {
              s.push(c);
            }
            None => todo!(),
          }
        }
      }
      Some('*') => {
        let mut s = String::new();

        loop {
          match chars.next() {
            Some('\r') => {
              match chars.next() {
                Some('\n') => {
                  match s.parse::<i64>() {
                    Ok(i) => {
                      if i == -1 {
                        return Ok(Resp::Array(vec![]));
                      } else if i >= 0 {
                        let mut a = vec![];

                        for _ in 0..i {
                          match Resp::decode(chars.clone().collect::<String>()) {
                            Ok(r) => {
                              a.push(r);
                            }
                            Err(e) => {
                              return Err(e);
                            }
                          }
                        }

                        return Ok(Resp::Array(a));
                      } else {
                        return Err("invalid array length".to_string());
                      }
                    }
                    Err(_) => {
                      return Err("196. expected integer".to_string());
                    }
                  }
                }
                _ => {
                  return Err("201. expected newline".to_string());
                }
              }
            }
            Some(c) => {
              s.push(c);
            }
            _ => {
              return Err("209. expected newline".to_string());
            }
          }
        }
      }
      _ => todo!(),
    }
  }
}