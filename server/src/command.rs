use crate::errors::ParseCommandError;
use crate::overrides::Result;

pub enum Command {
    Add { key: String, value: String }
}

impl Command {
    pub fn parse(input: &String) -> Result<Command> {
        let tokens = input.split(" ").collect::<Vec<&str>>();
        if tokens.len() != 3 {
            return Err(Box::new(ParseCommandError::MalformedInput));
        }
        return match tokens[0] {
            "ADD" => Ok(Command::Add { key: tokens[1].into(), value: tokens[2].into() }),
            _ => Err(Box::new(ParseCommandError::InvalidCommand { command: tokens[0].into() })),
        }
    }
}