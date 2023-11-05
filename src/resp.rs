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
                  return Err("expected newline".to_string());
                }
              }
            }
            Some(c) => {
              s.push(c);
            }
            None => {
              return Err("expected newline".to_string());
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
                  return Err("expected newline".to_string());
                }
              }
            }
            Some(c) => {
              s.push(c);
            }
            None => {
              return Err("expected newline".to_string());
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
                  return Err("expected newline".to_string());
                }
              }
            }
            Some(c) => {
              s.push(c);
            }
            None => {
              return Err("expected newline".to_string());
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
                        let buf = chars.take(i as usize).collect::<String>();

                        match chars.next() {
                          Some('\r') => {
                            match chars.next() {
                              Some('\n') => {
                                return Ok(Resp::BulkString(Some(s)));
                              }
                              _ => {
                                return Err("expected newline".to_string());
                              }
                            }
                          }
                          _ => {
                            return Err("expected newline".to_string());
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
                  return Err("expected newline".to_string());
                }
              }
            }
            Some(c) => {
              s.push(c);
            }
          }
        }
      }
    }
  }
}