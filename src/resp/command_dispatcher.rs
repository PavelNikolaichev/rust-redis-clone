use std::collections::HashMap;
/// Implementation of the RESP command dispatcher.
/// Simplifies the execution of RESP commands by providing a common interface.

use crate::resp::commands::{Command, Ping, Echo, DefaultServerState};
use crate::resp::protocol::RespType;

pub struct CommandDispatcher {
    pub commands: HashMap<String, Box<dyn Command + Send + Sync>>,
}

impl CommandDispatcher {
    pub fn new() -> Self {
        let mut commands: HashMap<String, Box<dyn Command + Send + Sync>> = HashMap::new();
        commands.insert("ECHO".to_string(), Box::new(Echo));
        commands.insert("PING".to_string(), Box::new(Ping));
        // Add more commands as needed

        Self { commands }
    }

    pub fn dispatch(&self, command_name: &str, args: Vec<RespType>, state: &mut DefaultServerState) -> Result<RespType, String> {
        if let Some(command) = self.commands.get(command_name.to_uppercase().as_str()) {
            command.execute(&args, state as &mut dyn crate::resp::commands::ServerState)
        } else {
            Err(format!("Unknown command: {}", command_name))
        }
    }
}