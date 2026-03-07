use crate::handlers::{handle_input_from_client, handle_output_to_client};
use crate::model::ServerState;
use std::collections::HashMap;
use std::io::{BufReader, BufWriter};
use std::net::{SocketAddr, TcpListener};
use std::sync::{Arc, Mutex, mpsc};
use std::thread;

pub struct Server {
    pub listener: TcpListener,
    pub state: Arc<Mutex<ServerState>>,
}

impl Server {
    pub fn new(port: u16) -> Result<Self, std::io::Error> {
        let socket_address = SocketAddr::from(([0, 0, 0, 0], port));
        let listener = TcpListener::bind(socket_address)?;

        let state = Arc::new(Mutex::new(ServerState {
            connections: HashMap::new(),
        }));

        Ok(Self { listener, state })
    }

    /*
       Levanta el servidor TCP con sockets.
    */
    pub fn run(&self) -> Result<(), std::io::Error> {
        println!(
            "Servidor {} escuchando en el puerto {}",
            self.listener.local_addr()?.ip(),
            self.listener.local_addr()?.port()
        );

        loop {
            match self.listener.accept() {
                Ok((socket, _)) => {
                    let socket_clone = socket.try_clone();
                    match socket_clone {
                        Ok(socket_clone) => {
                            let reader = BufReader::new(socket_clone);
                            let writer = BufWriter::new(socket);
                            let (sender, receiver) = mpsc::channel::<String>();
                            let state = Arc::clone(&self.state);

                            // TODO: incorporar a bitácora
                            println!("Conexion aceptada");
                            println!("\tCliente: {:?}", reader.get_ref().peer_addr());
                            println!("\tPuerto: {:?}", reader.get_ref().peer_addr()?.port());

                            // Manejar mensajes desde el cliente
                            thread::spawn(|| handle_input_from_client(reader, sender, state));

                            // Manejar mensajes hacia el cliente
                            thread::spawn(|| handle_output_to_client(writer, receiver));
                        }
                        Err(e) => {
                            eprintln!("Error clonando socket: {}", e);
                            continue;
                        }
                    }
                }
                _ => {
                    eprintln!("Error al aceptar conexión");
                    continue;
                }
            }
        }
    }
}
