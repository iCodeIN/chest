use std::collections::HashMap;

use crate::command::Command;

pub struct Store {
    map: HashMap<String, String>,
}

impl Store {
    pub fn new() -> Self {
        Self { map: HashMap::new() }
    }

    /// Executes a command against the store and returns a string that represents
    /// the result of the operation.
    pub fn execute_command(&mut self, command: &Command) -> String {
        match command {
            Command::Add { key, value } => {
                match self.map.insert(key.clone(), value.clone()) {
                    Some(_) => "0 Updated".into(),
                    None => "0 Added".into(),
                }
            },
            Command::Get { key } => {
                match self.map.get(key) {
                    Some(value) => format!("0 {}", value),
                    None => "2 Not Found".into(),
                }
            },
            Command::Remove { key } => {
                match self.map.remove(key) {
                    Some(_) => "0 Removed".into(),
                    None => "2 Not Found".into(),
                }
            },
        }
    }
}