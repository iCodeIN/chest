use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};

pub struct Server;

impl Server {
    pub fn new() -> Self {
        Self { }
    }

    pub fn listen(&self, port: u16) -> std::io::Result<()> {
        let listener = TcpListener::bind(format!("127.0.0.1:{}", port))?;
        for stream in listener.incoming() {
            handle_client(&mut stream?)?;
        }
        Ok(())
    }
}

fn handle_client(stream: &mut TcpStream) -> std::io::Result<()> {
    let mut input = [0; 4096];
    stream.read(&mut input)?;
    stream.write(&input)?;
    Ok(())
}