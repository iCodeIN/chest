use crate::command::Command;
use crate::overrides::Result;
use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

pub struct Server {
    storage: HashMap<String, String>,
}

impl Server {
    pub fn new() -> Self {
        Self { storage: HashMap::new() }
    }

    /// Starts the server, and begins listening for connections on the given port.
    pub fn listen(&mut self, port: u16) -> Result<()> {
        let listener = TcpListener::bind(format!("127.0.0.1:{}", port))?;
        for stream in listener.incoming() {
            match self.handle_request(&mut stream?) {
                Ok(()) => (),
                // TODO: Add some additional logging? Find a way to send an error to the user?
                Err(err) => {
                    println!("Error: {}", err)
                }
            }
        }
        Ok(())
    }

    /// Handles each request to the server.
    fn handle_request(&mut self, stream: &mut TcpStream) -> Result<()> {
        let mut input = [0; 4096];
        stream.read(&mut input)?;

        // TODO: Don't do unwrap here?
        let end = input.iter().position(|x| *x == 0).unwrap();
        let mut input = String::from(std::str::from_utf8(&input[0..end])?);
        input.pop();

        match Command::parse(&input)? {
            Command::Add { key, value } => {
                match self.storage.insert(key, value) {
                    Some(_) => stream.write("0 Updated".as_bytes())?,
                    None => stream.write("0 Added".as_bytes())?,
                }
            },
            Command::Get { key } => {
                match self.storage.get(&key) {
                    Some(value) => stream.write(format!("0 {}", value).as_bytes())?,
                    None => stream.write("2 Not Found".as_bytes())?,
                }
            },
            Command::Remove { key } => {
                match self.storage.remove(&key) {
                    Some(_) => stream.write("0 Removed".as_bytes())?,
                    None => stream.write("2 Not Found".as_bytes())?,
                }
            },
        };

        Ok(())
    }
}