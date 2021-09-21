use crate::command::Command;
use crate::overrides::Result;
use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::str;

pub struct Server {
    storage: HashMap<String, String>,
}

impl Server {
    pub fn new() -> Self {
        Self { storage: HashMap::new() }
    }

    pub fn listen(&mut self, port: u16) -> Result<()> {
        let listener = TcpListener::bind(format!("127.0.0.1:{}", port))?;
        for stream in listener.incoming() {
            match self.handle_client(&mut stream?) {
                Ok(()) => (),
                // TODO: Add some additional logging? Find a way to send an error to the user?
                Err(err) => {
                    println!("Error: {}", err)
                }
            }
        }
        Ok(())
    }

    fn handle_client(&mut self, stream: &mut TcpStream) -> Result<()> {
        let mut input = [0; 4096];
        stream.read(&mut input)?;
        match Command::parse(&String::from(str::from_utf8(&input)?))? {
            Command::Add { key, value } => {
                match self.storage.insert(key, value) {
                    Some(_) => stream.write("0 Updated".as_bytes())?,
                    None => stream.write("0 Added".as_bytes())?,
                }
            }
        };
        Ok(())
    }
}