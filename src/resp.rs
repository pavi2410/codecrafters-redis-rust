use core::slice::SlicePattern;
use std::io::Read;

use bytes::{Buf, BufMut};

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
      Resp::SimpleString(s) => {
        let mut buf = bytes::BytesMut::with_capacity(s.len() + 3);
        buf.put_slice(b"+");
        buf.put_slice(s.as_bytes());
        buf.put_slice(b"\r\n");
        &buf
      },
      Resp::Error(s) => {
        let mut buf = bytes::BytesMut::with_capacity(s.len() + 3);
        buf.put_slice(b"-");
        buf.put_slice(s.as_bytes());
        buf.put_slice(b"\r\n");
        &buf
      },
      Resp::Integer(i) => {
        let mut buf = bytes::BytesMut::with_capacity(20 + 3);
        buf.put_slice(b":");
        buf.put_slice(i.to_string().as_bytes());
        buf.put_slice(b"\r\n");
        &buf
      },
      Resp::BulkString(Some(s)) => {
        let mut buf = bytes::BytesMut::with_capacity(s.len() + 3 + 3);
        buf.put_slice(b"$");
        buf.put_slice(s.len().to_string().as_bytes());
        buf.put_slice(b"\r\n");
        buf.put_slice(s.as_bytes());
        buf.put_slice(b"\r\n");
        &buf
      },
      Resp::BulkString(None) => {
        let mut buf = bytes::BytesMut::with_capacity(3);
        buf.put_slice(b"$-1\r\n");
        &buf
      },
      Resp::Array(a) => {
        let mut buf = bytes::BytesMut::with_capacity(3 + 3);

        buf.put_slice(b"*");
        buf.put_slice(a.len().to_string().as_bytes());
        buf.put_slice(b"\r\n");

        for s in a {
          buf.put_slice(s.encode());
        }

        &buf
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