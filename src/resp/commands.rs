use crate::resp::protocol::RespType;

/// This module defines the RESP commands and their serialization/deserialization logic.
/// Utilizes Strategy pattern for command handling.

pub trait ServerState {
    /// Simple interface for redis server state.

    fn get(&mut self, key: &str) -> Option<RespType>;
    fn set(&mut self, key: String, value: RespType) -> Result<(), String>;

    fn del(&mut self, key: &str) -> Result<(), String>;

    fn exists(&mut self, key: &str) -> bool;

    fn flush(&mut self) -> Result<(), String>;

    fn keys(&mut self) -> Vec<String>;

    fn get_all(&mut self) -> Vec<(String, RespType)>;

    fn incr(&mut self, key: &str) -> Result<i64, String>;

    fn decr(&mut self, key: &str) -> Result<i64, String>;

    fn expire(&mut self, key: &str, seconds: u64) -> Result<(), String>;

    fn ttl(&mut self, key: &str) -> Result<Option<u64>, String>;

    fn persist(&mut self, key: &str) -> Result<(), String>;

    fn type_of(&mut self, key: &str) -> Result<RespType, String>;

    fn rename(&mut self, old_key: &str, new_key: &str) -> Result<(), String>;

    fn rename_if_exists(&mut self, old_key: &str, new_key: &str) -> Result<(), String>;

    fn append(&mut self, key: &str, value: &str) -> Result<RespType, String>;

    fn get_range(&mut self, key: &str, start: i64, end: i64) -> Result<RespType, String>;

    fn set_range(&mut self, key: &str, offset: i64, value: &str) -> Result<RespType, String>;

    fn get_set(&mut self, key: &str, value: &str) -> Result<RespType, String>;
}

pub struct DefaultServerState {
    // This is a placeholder for the actual server state implementation.
    // In a real application, this would manage the data store.
    data: std::collections::HashMap<String, RespType>,
}

impl Default for DefaultServerState {
    fn default() -> Self {
        DefaultServerState {
            data: std::collections::HashMap::new(),
        }
    }
}

impl ServerState for DefaultServerState {
    fn get(&mut self, key: &str) -> Option<RespType> {
        self.data.get(key).cloned()
    }

    fn set(&mut self, key: String, value: RespType) -> Result<(), String> {
        self.data.insert(key, value);
        Ok(())
    }

    fn del(&mut self, key: &str) -> Result<(), String> {
        self.data.remove(key);
        Ok(())
    }

    fn exists(&mut self, key: &str) -> bool {
        self.data.contains_key(key)
    }

    fn flush(&mut self) -> Result<(), String> {
        self.data.clear();
        Ok(())
    }

    fn keys(&mut self) -> Vec<String> {
        self.data.keys().cloned().collect()
    }

    fn get_all(&mut self) -> Vec<(String, RespType)> {
        self.data
            .iter()
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect()
    }

    fn incr(&mut self, key: &str) -> Result<i64, String> {
        if let Some(RespType::Integer(value)) = self.data.get(key) {
            let new_value = value + 1;
            self.data
                .insert(key.to_string(), RespType::Integer(new_value));
            Ok(new_value)
        } else {
            Err("Key does not exist or is not an integer".to_string())
        }
    }

    fn decr(&mut self, key: &str) -> Result<i64, String> {
        if let Some(RespType::Integer(value)) = self.data.get(key) {
            let new_value = value - 1;
            self.data
                .insert(key.to_string(), RespType::Integer(new_value));
            Ok(new_value)
        } else {
            Err("Key does not exist or is not an integer".to_string())
        }
    }

    fn expire(&mut self, key: &str, _seconds: u64) -> Result<(), String> {
        // This is a placeholder implementation. In a real application, you would need to handle expiration.
        // Here we just return Ok to indicate success.
        if self.data.contains_key(key) {
            Ok(())
        } else {
            Err("Key does not exist".to_string())
        }
    }

    fn ttl(&mut self, key: &str) -> Result<Option<u64>, String> {
        // This is a placeholder implementation. In a real application, you would need to handle expiration.
        // Here we just return None to indicate no expiration.
        if self.data.contains_key(key) {
            Ok(Some(0)) // Indicating no expiration
        } else {
            Err("Key does not exist".to_string())
        }
    }

    fn persist(&mut self, key: &str) -> Result<(), String> {
        // This is a placeholder implementation. In a real application, you would need to handle persistence.
        // Here we just return Ok to indicate success.
        if self.data.contains_key(key) {
            Ok(())
        } else {
            Err("Key does not exist".to_string())
        }
    }

    fn type_of(&mut self, key: &str) -> Result<RespType, String> {
        if let Some(value) = self.data.get(key) {
            match value {
                RespType::SimpleString(_) => Ok(RespType::SimpleString("string".to_string())),
                RespType::Integer(_) => Ok(RespType::SimpleString("integer".to_string())),
                RespType::BulkString(_) => Ok(RespType::SimpleString("bulk_string".to_string())),
                RespType::Array(_) => Ok(RespType::SimpleString("array".to_string())),
                RespType::Error(_) => Ok(RespType::SimpleString("error".to_string())),
            }
        } else {
            Err("Key does not exist".to_string())
        }
    }

    fn rename(&mut self, old_key: &str, new_key: &str) -> Result<(), String> {
        if let Some(value) = self.data.remove(old_key) {
            self.data.insert(new_key.to_string(), value);
            Ok(())
        } else {
            Err("Key does not exist".to_string())
        }
    }

    fn rename_if_exists(&mut self, old_key: &str, new_key: &str) -> Result<(), String> {
        if self.data.contains_key(old_key) {
            if self.data.contains_key(new_key) {
                return Err("New key already exists".to_string());
            }
            let value = self.data.remove(old_key).unwrap();
            self.data.insert(new_key.to_string(), value);
            Ok(())
        } else {
            Err("Old key does not exist".to_string())
        }
    }

    fn append(&mut self, key: &str, value: &str) -> Result<RespType, String> {
        match self.data.get_mut(key) {
            Some(RespType::BulkString(existing_value)) => {
                let new_value = match existing_value {
                    Some(s) => {
                        s.push_str(value);
                        s.clone()
                    }
                    None => value.to_string(),
                };
                *existing_value = Some(new_value.clone());
                Ok(RespType::Integer(new_value.len() as i64))
            }
            None => {
                self.data.insert(
                    key.to_string(),
                    RespType::BulkString(Some(value.to_string())),
                );
                Ok(RespType::Integer(value.len() as i64))
            }
            _ => Err("Key exists but is not a bulk string".to_string()),
        }
    }

    fn get_range(&mut self, key: &str, start: i64, end: i64) -> Result<RespType, String> {
        if let Some(RespType::BulkString(Some(value))) = self.data.get(key) {
            let start = start.max(0) as usize;
            let end = end.max(0) as usize;
            let range_value = &value[start..end.min(value.len())];
            Ok(RespType::BulkString(Some(range_value.to_string())))
        } else {
            Err("Key does not exist or is not a bulk string".to_string())
        }
    }

    fn set_range(&mut self, key: &str, offset: i64, value: &str) -> Result<RespType, String> {
        if offset < 0 {
            return Err("Offset is out of range".to_string());
        }
        let offset = offset as usize;
        let new_value = match self.data.get_mut(key) {
            Some(RespType::BulkString(existing_value)) => {
                let mut s = existing_value.clone().unwrap_or_default();
                if offset > s.len() {
                    s.push_str(&" ".repeat(offset - s.len()));
                }
                if offset + value.len() > s.len() {
                    s.push_str(&" ".repeat(offset + value.len() - s.len()));
                }
                s.replace_range(offset..offset + value.len(), value);
                *existing_value = Some(s.clone());
                s
            }
            None => {
                let mut s = String::new();
                if offset > 0 {
                    s.push_str(&" ".repeat(offset));
                }
                s.push_str(value);
                self.data
                    .insert(key.to_string(), RespType::BulkString(Some(s.clone())));
                s
            }
            _ => return Err("Key exists but is not a bulk string".to_string()),
        };
        Ok(RespType::Integer(new_value.len() as i64))
    }

    fn get_set(&mut self, key: &str, value: &str) -> Result<RespType, String> {
        if let Some(existing_value) = self.data.remove(key) {
            self.data.insert(
                key.to_string(),
                RespType::BulkString(Some(value.to_string())),
            );
            Ok(existing_value)
        } else {
            Err("Key does not exist".to_string())
        }
    }
}

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
