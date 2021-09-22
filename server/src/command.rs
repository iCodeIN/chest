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
        return match tokens[0] {
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
}