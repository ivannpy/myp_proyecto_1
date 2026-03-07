mod server;
mod connection;
mod utils;
mod model;
mod handlers;

use crate::server::Server;

/*
    Punto de entrada del programa.
 */
fn main() {
    let port: u16 = 1234;

    let server = Server::new(port);
    server.and_then(|s| s.run()).expect("No se pudo iniciar");
}