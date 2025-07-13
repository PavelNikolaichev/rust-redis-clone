use std::fmt::Debug;
use std::io::{Error, ErrorKind};

/// A Rust implementation of the RESP 2 protocol.
/// This module defines the RESP protocol types
pub enum RespType {
    SimpleString(String),
    Error(String),
    Integer(i64),
    BulkString(Option<String>), // None for incomplete bulk strings
    Array(Vec<RespType>),
}

impl Clone for RespType {
    fn clone(&self) -> Self {
        match self {
            RespType::SimpleString(s) => RespType::SimpleString(s.clone()),
            RespType::Error(e) => RespType::Error(e.clone()),
            RespType::Integer(i) => RespType::Integer(*i),
            RespType::BulkString(s) => RespType::BulkString(s.clone()),
            RespType::Array(arr) => RespType::Array(arr.clone()),
        }
    }
}

impl PartialEq<RespType> for RespType {
    fn eq(&self, other: &RespType) -> bool {
        match (self, other) {
            (RespType::SimpleString(s1), RespType::SimpleString(s2)) => s1 == s2,
            (RespType::Error(e1), RespType::Error(e2)) => e1 == e2,
            (RespType::Integer(i1), RespType::Integer(i2)) => i1 == i2,
            (RespType::BulkString(Some(s1)), RespType::BulkString(Some(s2))) => s1 == s2,
            (RespType::BulkString(None), RespType::BulkString(None)) => true,
            (RespType::Array(a1), RespType::Array(a2)) => a1 == a2,
            _ => false,
        }
    }
}

impl Debug for RespType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RespType::SimpleString(s) => write!(f, "SimpleString({})", s),
            RespType::Error(e) => write!(f, "Error({})", e),
            RespType::Integer(i) => write!(f, "Integer({})", i),
            RespType::BulkString(Some(s)) => write!(f, "BulkString({})", s),
            RespType::BulkString(None) => write!(f, "BulkString(None)"),
            RespType::Array(arr) => write!(f, "Array({:?})", arr),
        }
    }
}

pub fn serialize(resp: &RespType) -> Vec<u8> {
    match resp {
        RespType::SimpleString(s) => format!("+{}\r\n", s).into_bytes(),
        RespType::Error(s) => format!("-{}\r\n", s).into_bytes(),
        RespType::Integer(x) => format!(":{}\r\n", x).into_bytes(),
        RespType::BulkString(s) => {
            if let Some(ref s) = s {
                format!("${}\r\n{}\r\n", s.len(), s).into_bytes()
            } else {
                b"$-1\r\n".to_vec() // Incomplete bulk string
            }
        }
        RespType::Array(arr) => {
            let mut res = format!("*{}\r\n", arr.len()).into_bytes();

            for item in arr {
                res.extend(serialize(item));
            }

            res
        }
    }
}

/// helper to pull a line until "\r\n", returning (line_without_crlf, bytes_consumed)
fn parse_line(input: &[u8]) -> Result<(&[u8], usize), Error> {
    if let Some(pos) = input.windows(2).position(|w| w == b"\r\n") {
        Ok((&input[..pos], pos + 2))
    } else {
        Err(Error::new(ErrorKind::UnexpectedEof, "No CRLF found"))
    }
}

pub fn deserialize(input: &[u8]) -> Result<(RespType, usize), Error> {
    if input.is_empty() {
        return Err(Error::new(ErrorKind::UnexpectedEof, "Empty input"));
    }
    match input[0] {
        b'+' => {
            let (line, n) = parse_line(&input[1..])?;
            let s = std::str::from_utf8(line)
                .map_err(|e| Error::new(ErrorKind::InvalidData, e))?
                .to_string();

            Ok((RespType::SimpleString(s), 1 + n))
        }
        b'-' => {
            let (line, n) = parse_line(&input[1..])?;
            let s = std::str::from_utf8(line)
                .map_err(|e| Error::new(ErrorKind::InvalidData, e))?
                .to_string();

            Ok((RespType::Error(s), 1 + n))
        }
        b':' => {
            let (line, n) = parse_line(&input[1..])?;
            let num = std::str::from_utf8(line)
                .map_err(|e| Error::new(ErrorKind::InvalidData, e))?
                .parse::<i64>()
                .map_err(|e| Error::new(ErrorKind::InvalidData, e))?;

            Ok((RespType::Integer(num), 1 + n))
        }
        b'$' => {
            let (line, header) = parse_line(&input[1..])?;
            let len = std::str::from_utf8(line)
                .map_err(|e| Error::new(ErrorKind::InvalidData, e))?
                .parse::<isize>()
                .map_err(|e| Error::new(ErrorKind::InvalidData, e))?;

            if len < 0 {
                Ok((RespType::BulkString(None), 1 + header))
            } else {
                let total = 1 + header + len as usize + 2;

                if input.len() < total {
                    return Err(Error::new(
                        ErrorKind::UnexpectedEof,
                        "Incomplete bulk string",
                    ));
                }

                let data = &input[1 + header..1 + header + len as usize];
                let s = std::str::from_utf8(data)
                    .map_err(|e| Error::new(ErrorKind::InvalidData, e))?
                    .to_string();

                Ok((RespType::BulkString(Some(s)), total))
            }
        }
        b'*' => {
            let (line, header) = parse_line(&input[1..])?;
            let count = std::str::from_utf8(line)
                .map_err(|e| Error::new(ErrorKind::InvalidData, e))?
                .parse::<isize>()
                .map_err(|e| Error::new(ErrorKind::InvalidData, e))?;

            if count < 0 {
                Ok((RespType::Array(Vec::new()), 1 + header))
            } else {
                let mut items = Vec::with_capacity(count as usize);
                let mut offset = 1 + header;

                for _ in 0..count {
                    let (item, n) = deserialize(&input[offset..])?;
                    items.push(item);
                    offset += n;
                }
                Ok((RespType::Array(items), offset))
            }
        }
        _ => Err(Error::new(ErrorKind::InvalidData, "Unknown RESP type byte")),
    }
}

// impl PartialEq for (RespType, usize) {
//     fn eq(&self, other: &(RespType, usize)) -> bool {
//         self.0 == other.0 && self.1 == other.1
//     }
// }
//
// impl Debug for (RespType, usize) {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         f.debug_tuple("")
//             .field(&self.0)
//             .field(&self.1)
//             .finish()
//     }
// }
