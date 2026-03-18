mod handlers;
mod model;
mod network_connection;
pub mod server;

use server::Server;
use std::env;
use std::process;

/// Obtiene el puerto del servidor desde los argumentos de línea de comandos
/// o usa el puerto 1234 por defecto.
///
/// Regresa el puerto del servidor
fn get_port() -> u16 {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        1234
    } else {
        if args.len() != 2 {
            println!("Uso: {} <puerto>", args[0]);
            process::exit(1);
        }
        match args[1].parse() {
            Ok(p) => p,
            Err(_) => {
                eprintln!("Error: '{}' no es un puerto válido", args[1]);
                process::exit(1);
            }
        }
    }
}

/// Punto de entrada del servidor
fn main() {
    let port: u16 = get_port();

    let server = Server::new(port);
    server.and_then(|s| s.run()).expect("No se pudo iniciar");
}
