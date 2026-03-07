mod server_config;
mod server;
mod connection;
mod utils;
mod protocol;

use crate::server::Server;
use crate::server_config::{get_config, ServerConfig};

/*
    Punto de entrada del programa.
 */
fn main() {
    let config: ServerConfig = get_config();

    let address: [u8; 4] = config.get_host();
    let port: u16 = config.get_port();

    let mut server: Server = Server::new(address, port);
    server.start().unwrap();
}