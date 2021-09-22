use std::collections::HashMap;
use std::fmt;

use crate::errors::ParseCommandError;
use crate::overrides::Result;

pub enum Command {
    Add { key: String, value: String },
    Get { key: String },
    Remove { key: String },
}

impl Command {
    /// Parses the given string into a Command type.
    pub fn parse(input: &String) -> Result<Command> {
        let tokens = input.split(" ").collect::<Vec<&str>>();
        match tokens[0] {
            "ADD" => {
                if tokens.len() != 3 {
                    return Err(Box::new(ParseCommandError::MalformedInput));
                }
                Ok(Command::Add { key: tokens[1].into(), value: tokens[2].into() })
            },
            "GET" => {
                if tokens.len() != 2 {
                    return Err(Box::new(ParseCommandError::MalformedInput));
                }
                Ok(Command::Get { key: tokens[1].into() })
            },
            "REMOVE" => {
                if tokens.len() != 2 {
                    return Err(Box::new(ParseCommandError::MalformedInput));
                }
                Ok(Command::Remove { key: tokens[1].into() })
            }
            _ => Err(Box::new(ParseCommandError::InvalidCommand { command: tokens[0].into() })),
        }
    }

    /// Executes the command against the given key-value store.
    pub fn execute(&self, store: &mut HashMap<String, String>) -> CommandExecutionResult {
        match self {
            Command::Add { key, value } => {
                match store.insert(key.clone(), value.clone()) {
                    Some(_) => CommandExecutionResult::Updated,
                    None => CommandExecutionResult::Added,
                }
            },
            Command::Get { key } => {
                match store.get(key) {
                    Some(value) => CommandExecutionResult::Got { value: value.clone() },
                    None => CommandExecutionResult::NotFound,
                }
            },
            Command::Remove { key } => {
                match store.remove(key) {
                    Some(_) => CommandExecutionResult::Removed,
                    None => CommandExecutionResult::NotFound,
                }
            },
        }
    }
}

pub enum CommandExecutionResult {
    Added,
    Got { value: String },
    Removed,
    Updated,
    NotFound,
}

impl fmt::Display for CommandExecutionResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Added => write!(f, "0 Added"),
            Self::Got { value } => write!(f, "0 {}", value),
            Self::Removed => write!(f, "0 Removed"),
            Self::Updated => write!(f, "0 Updated"),
            Self::NotFound => write!(f, "2 Not Found"),
        }
    }
}
