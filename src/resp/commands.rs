use crate::resp::protocol::RespType;
use crate::resp::state::server_state::ServerState;

/// This module defines the RESP commands and their serialization/deserialization logic.
/// Utilizes Strategy pattern for command handling.

pub trait Command: Send + Sync + 'static {
    fn name(&self) -> &str;
    fn execute(&self, args: &[RespType], state: &mut dyn ServerState) -> Result<RespType, String>;
}

pub struct Ping;

impl Command for Ping {
    fn name(&self) -> &str {
        "PING"
    }

    fn execute(
        &self,
        _args: &[RespType],
        _state: &mut dyn ServerState,
    ) -> Result<RespType, String> {
        Ok(RespType::SimpleString("PONG".to_string()))
    }
}

pub struct Echo;

impl Command for Echo {
    fn name(&self) -> &str {
        "ECHO"
    }

    fn execute(&self, args: &[RespType], _state: &mut dyn ServerState) -> Result<RespType, String> {
        if args.len() < 1 {
            Err("ECHO requires at least one argument".to_string())
        } else {
            match &args[0] {
                RespType::BulkString(Some(value)) => Ok(RespType::BulkString(Some(value.clone()))),
                RespType::SimpleString(value) => Ok(RespType::SimpleString(value.clone())),
                _ => Err("ECHO argument must be a string".to_string()),
            }
        }
    }
}

pub struct Set;
impl Command for Set {
    fn name(&self) -> &str {
        "SET"
    }

    fn execute(&self, args: &[RespType], state: &mut dyn ServerState) -> Result<RespType, String> {
        if args.len() < 2 {
            return Err("SET requires at least two arguments".to_string());
        }

        let mut ttl: Option<i64> = None;
        if args.len() > 3 {
            match args[2] {
                RespType::SimpleString(ref option) => {
                    if option != "PX" {
                        return Err("Invalid SET option, expected 'PX'".to_string());
                    }

                    match &args[3] {
                        RespType::BulkString(Some(ttl_str)) | RespType::SimpleString(ttl_str) => {
                            match ttl_str.parse::<i64>() {
                                Ok(parsed_ttl) if parsed_ttl >= 0 => ttl = Some(parsed_ttl),
                                Ok(_) => {
                                    return Err("TTL must be a non-negative integer".to_string())
                                }
                                Err(_) => return Err("TTL must be an integer".to_string()),
                            }
                        }
                        _ => return Err("TTL must be a string".to_string()),
                    }
                }
                RespType::BulkString(Some(ref option)) if option.to_uppercase() == "PX" => {
                    match &args[3] {
                        RespType::BulkString(Some(ttl_str)) | RespType::SimpleString(ttl_str) => {
                            match ttl_str.parse::<i64>() {
                                Ok(parsed_ttl) if parsed_ttl >= 0 => ttl = Some(parsed_ttl),
                                Ok(_) => {
                                    return Err("TTL must be a non-negative integer".to_string())
                                }
                                Err(_) => return Err("TTL must be an integer".to_string()),
                            }
                        }
                        _ => return Err("TTL must be a string".to_string()),
                    }
                }
                _ => {
                    return Err("Invalid SET option, expected 'PX'".to_string());
                }
            }
        }

        match &args[0] {
            RespType::BulkString(Some(key)) => {
                let value = args[1].clone();
                match state.set(key.clone(), value, ttl) {
                    Ok(_) => Ok(RespType::SimpleString("OK".to_string())),
                    Err(_) => Err("Failed to set value".to_string()),
                }
            }
            _ => Err("SET key and value must be strings".to_string()),
        }
    }
}

pub struct Get;
impl Command for Get {
    fn name(&self) -> &str {
        "GET"
    }

    fn execute(&self, args: &[RespType], state: &mut dyn ServerState) -> Result<RespType, String> {
        if args.len() < 1 {
            Err("GET requires at least one argument".to_string())
        } else {
            match &args[0] {
                RespType::BulkString(Some(key)) => match state.get(key) {
                    Some(value) => Ok(value),
                    None => Ok(RespType::BulkString(None)),
                },
                _ => Err("GET key must be a string".to_string()),
            }
        }
    }
}
