mod server_config;
mod server;
mod connection;
mod utils;
mod model;

use crate::server::Server;
use crate::server_config::{get_config, ServerConfig};

/*
    Punto de entrada del programa.
 */
fn main() {
    let config: ServerConfig = get_config();

    let port: u16 = config.get_port();

    let server = Server::new(port);
    server.and_then(|mut s| s.run()).expect("No se pudo iniciar");
}