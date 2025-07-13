use crate::resp::protocol::RespType;
use crate::resp::state::server_state::ServerState;
use log::{info};

// TODO: handle active expiration
pub struct DefaultServerState {
    // This is a placeholder for the actual server state implementation.
    // In a real application, this would manage the data store.
    data: std::collections::HashMap<String, RespType>,

    expires: std::collections::HashMap<String, u64>, // Placeholder for expiration times
}

impl Default for DefaultServerState {
    fn default() -> Self {
        DefaultServerState {
            data: std::collections::HashMap::new(),
            expires: std::collections::HashMap::new(),
        }
    }
}

impl ServerState for DefaultServerState {
    fn get(&mut self, key: &str) -> Option<RespType> {
        info!("Getting key: {}", key);
        if let Some(expiration) = self.expires.get(key) {
            let current_time = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map_err(|_| "Failed to get current time".to_string())
                .ok()?
                .as_millis() as u64;

            info!(
                "Checking expiration for key: {}, Expiration: {}, Current Time: {}",
                key, expiration, current_time
            );

            if *expiration <= current_time {
                self.data.remove(key);
                self.expires.remove(key);
                return None;
            }
        }

        self.data.get(key).cloned()
    }

    fn set(&mut self, key: String, value: RespType, ttl: Option<i64>) -> Result<(), String> {
        self.data.insert(key.clone(), value);
        info!("Setting key: {}, value: {:?}", key, self.data.get(&key));
        if let Some(milliseconds) = ttl {
            if milliseconds < 0 {
                return Err("TTL cannot be negative".to_string());
            }

            let current_time = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map_err(|_| "Failed to get current time".to_string())?
                .as_millis() as u64;

            info!(
                "Setting expiration for key: {}, TTL: {} ms",
                key, milliseconds
            );

            self.expires
                .insert(key, current_time + milliseconds as u64);
        }
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
        if self.data.contains_key(key) {
            let current_time = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map_err(|_| "Failed to get current time".to_string())?
                .as_secs();

            self.expires
                .insert(key.to_string(), current_time + _seconds);

            Ok(())
        } else {
            Err("Key does not exist".to_string())
        }
    }

    fn ttl(&mut self, key: &str) -> Result<Option<u64>, String> {
        if let Some(expiration) = self.expires.get(key) {
            let current_time = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map_err(|_| "Failed to get current time".to_string())?
                .as_secs();

            if *expiration > current_time {
                Ok(Some(*expiration - current_time))
            } else {
                Ok(Some(0))
            }
        } else {
            Ok(None)
        }
    }

    fn persist(&mut self, key: &str) -> Result<(), String> {
        if self.data.contains_key(key) {
            self.expires.remove(key);
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
