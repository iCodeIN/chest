mod command;
mod errors;
mod overrides;
mod server;

use overrides::Result;
use server::Server;

const DEFAULT_PORT: u16 = 9077;

fn main() -> Result<()> {
    Server::new().listen(DEFAULT_PORT)?;
    Ok(())
}
