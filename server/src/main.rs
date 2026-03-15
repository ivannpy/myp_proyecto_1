mod broadcaster;
mod handlers;
mod model;

use model::server::Server;
use std::env;
use std::process;

///
/// Punto de entrada del servidor
///
fn main() {
    let args: Vec<String> = env::args().collect();
    let port: u16;
    
    if args.len() == 1 {
        port = 1234;
    } else {
        if args.len() != 2 {
            println!("Uso: {} <puerto>", args[0]);
            process::exit(1);
        }
        port = match args[1].parse() {
            Ok(p) => p,
            Err(_) => {
                eprintln!("Error: '{}' no es un puerto válido", args[1]);
                process::exit(1);
            }
        };
    }
    
    let server = Server::new(port);
    server.and_then(|s| s.run()).expect("No se pudo iniciar");
}
