mod server;
use server::Server;

const DEFAULT_PORT: u16 = 9077;

fn main() -> std::io::Result<()> {
    Server::new().listen(DEFAULT_PORT)?;
    Ok(())
}
