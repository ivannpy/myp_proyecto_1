mod broadcaster;
mod handlers;
mod model;

use model::server::Server;
use std::env;
use std::process;

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

///
/// Punto de entrada del servidor
///
fn main() {
    let port: u16 = get_port();

    let server = Server::new(port);
    server.and_then(|s| s.run()).expect("No se pudo iniciar");
}
