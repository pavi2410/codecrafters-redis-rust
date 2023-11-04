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

  pub fn decode(s: &str) -> Result<Resp, String> {
    // decode char by char
    let mut chars = s.chars();

    // get the first char
    let c = chars.next().ok_or("incomplete")?;

    match c {
      '+' => {
        let mut s = String::new();

        loop {
          let c = chars.next().ok_or("incomplete")?;

          if c == '\r' {
            break;
          } else {
            s.push(c);
          }
        }

        let c = chars.next().ok_or("incomplete")?;

        if c == '\n' {
          Ok(Resp::SimpleString(s))
        } else {
          Err("expected newline".to_string())
        }
      }
      '-' => {
        let mut s = String::new();

        loop {
          let c = chars.next().ok_or("incomplete")?;

          if c == '\r' {
            break;
          } else {
            s.push(c);
          }
        }

        let c = chars.next().ok_or("incomplete")?;

        if c == '\n' {
          Ok(Resp::Error(s))
        } else {
          Err("expected newline".to_string())
        }
      }
      ':' => {
        let mut s = String::new();

        loop {
          let c = chars.next().ok_or("incomplete")?;

          if c == '\r' {
            break;
          } else {
            s.push(c);
          }
        }

        let c = chars.next().ok_or("incomplete")?;

        if c == '\n' {
          Ok(Resp::Integer(s.parse::<i64>().map_err(|_| "invalid integer")?))
        } else {
          Err("expected newline".to_string())
        }
      }
      '$' => {
        let mut s = String::new();

        loop {
          let c = chars.next().ok_or("incomplete")?;

          if c == '\r' {
            break;
          } else {
            s.push(c);
          }
        }

        let c = chars.next().ok_or("incomplete")?;

        if c == '\n' {
          let len = s.parse::<i64>().map_err(|_| "invalid integer")?;

          if len == -1 {
            Ok(Resp::BulkString(None))
          } else {
            let mut s = String::new();

            for _ in 0..len {
              let c = chars.next().ok_or("incomplete")?;

              s.push(c);
            }

            let c = chars.next().ok_or("incomplete")?;

            if c == '\r' {
              let c = chars.next().ok_or("incomplete")?;

              if c == '\n' {
                Ok(Resp::BulkString(Some(s)))
              } else {
                Err("expected newline".to_string())
              }
            } else {
              Err("expected newline".to_string())
            }
          }
        } else {
          Err("expected newline".to_string())
        }
      }
      '*' => {
        let mut s = String::new();

        loop {
          let c = chars.next().ok_or("incomplete")?;

          if c == '\r' {
            break;
          } else {
            s.push(c);
          }
        }

        let c = chars.next().ok_or("incomplete")?;

        if c == '\n' {
          let len = s.parse::<i64>().map_err(|_| "invalid integer")?;

          let mut a = Vec::new();

          for _ in 0..len {
            a.push(Resp::decode(&chars.as_str())?);
          }

          Ok(Resp::Array(a))
        } else {
          Err("expected newline".to_string())
        }
      }
      _ => Err("invalid type prefix".to_string()),
    }
  }
}