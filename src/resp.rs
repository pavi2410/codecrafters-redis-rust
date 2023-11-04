pub enum Resp {
    SimpleString(String),
    Error(String),
    Integer(i64),
    BulkString(Option<String>),
    Array(Vec<Resp>),
}

impl Resp {
  pub fn encode(&self) -> &[u8] {
    match self {
      Resp::SimpleString(s) => format!("+{}\r\n", s).as_bytes(),
      Resp::Error(s) => format!("-{}\r\n", s).as_bytes(),
      Resp::Integer(i) => format!(":{}\r\n", i).as_bytes(),
      Resp::BulkString(Some(s)) => format!("${}\r\n{}\r\n", s.len(), s).as_bytes(),
      Resp::BulkString(None) => "$-1\r\n".as_bytes(),
      Resp::Array(a) => {
        let mut s = format!("*{}\r\n", a.len());

        for r in a {
          s.push_str(&String::from_utf8_lossy(r.encode()));
        }

        s.as_bytes()
      },
    }
  }

  pub fn decode(s: &str) -> Result<Resp, String> {
    let mut chars = s.chars();
    let first_char = chars.next().ok_or("empty string")?;

    match first_char {
      '+' => Ok(Resp::SimpleString(chars.collect())),
      '-' => Ok(Resp::Error(chars.collect())),
      ':' => Ok(Resp::Integer(chars.collect::<String>().parse::<i64>().map_err(|e| e.to_string())?)),
      '$' => {
        // what is the pattern?
        // A: $<number of bytes>\r\n<bytes>\r\n

        let parts = s.strip_prefix('$').unwrap().split("\r\n").collect::<Vec<_>>();

        let len = parts[0].parse::<i64>().map_err(|e| e.to_string())?;
        let bytes = parts[1][..len as usize].to_string();

        Ok(Resp::BulkString(Some(bytes)))
      },
      '*' => {
        // what is the pattern?
        // A: *<number of elements in array>\r\n<element1>\r\n<element2>\r\n...

        let parts = s.strip_prefix('*').unwrap().split("\r\n").collect::<Vec<_>>();

        let len = parts[0].parse::<i64>().map_err(|e| e.to_string())?;
        let mut array = Vec::new();

        for i in 0..len {
          let part = parts[i as usize + 1];
          array.push(Resp::decode(part)?);
        }

        Ok(Resp::Array(array))
      },
      _ => Err(format!("unknown type: {}", first_char)),
    }
  }
}