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
    let mut lines = s.lines();

    let line = lines.next().unwrap();

    match line.chars().next().unwrap() {
      '+' => Ok(Resp::SimpleString(line[1..].to_string())),
      '-' => Ok(Resp::Error(line[1..].to_string())),
      ':' => Ok(Resp::Integer(line[1..].parse().unwrap())),
      '$' => {
        let len = line[1..].parse::<i32>().unwrap();

        if len == -1 {
          Ok(Resp::BulkString(None))
        } else {
          let s = lines.next().unwrap();

          Ok(Resp::BulkString(Some(s.to_string())))
        }
      }
      '*' => {
        let len = line[1..].parse::<i32>().unwrap();

        let mut a = Vec::new();

        for _ in 0..len {
          let line = lines.next().unwrap();

          a.push(Resp::decode(line)?);
        }

        Ok(Resp::Array(a))
      }
      _ => Err("unknown type".to_string()),
    }
  }
}