mod broadcaster;
mod handlers;
mod model;

use model::server::Server;

///
/// Punto de entrada del servidor
///
fn main() {
    let port: u16 = 1234;

    let server = Server::new(port);
    server.and_then(|s| s.run()).expect("No se pudo iniciar");
}
