use crate::server::Server;

mod server_config;
mod server;


/*
    Punto de entrada del programa.
 */
fn main() {
    let address: [u8; 4] = [127, 0, 0, 1];
    let port: u16 = 7878;

    let server: Server = Server::new(address, port);
    server.start().unwrap();
}