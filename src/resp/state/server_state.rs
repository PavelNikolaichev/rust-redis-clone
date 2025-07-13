use crate::resp::protocol::RespType;

/// Simple interface for redis server state.
pub trait ServerState {
    fn get(&mut self, key: &str) -> Option<RespType>;
    fn set(&mut self, key: String, value: RespType, ttl: Option<i64>) -> Result<(), String>;

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