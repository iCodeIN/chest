use std::fmt;

#[derive(Debug)]
pub enum ParseCommandError {
    InvalidCommand { command: String },
    MalformedInput,
}

impl std::error::Error for ParseCommandError { }

impl std::fmt::Display for ParseCommandError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &*self {
            Self::InvalidCommand { command } => write!(f, "Invalid command \"{}\" provided.", command),
            Self::MalformedInput => write!(f, "Malformed input."),
        }
    }
}