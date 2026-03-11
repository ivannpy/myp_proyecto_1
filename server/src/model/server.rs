use crate::broadcaster::Broadcaster;
use crate::handlers::{ClientHandler, handle_input_from_client, handle_output_to_client};
use crate::model::server_state::ServerState;
use protocol::messages::client_message::ClientMessage;
use std::io::{BufReader, BufWriter};
use std::net::{SocketAddr, TcpListener};
use std::sync::{Arc, Mutex, mpsc};
use std::thread;

/*
   Representa un servidor de sockets TCP.
*/
pub struct Server {
    pub listener: TcpListener,
    pub state: Arc<Mutex<ServerState>>,
    pub broadcaster: Arc<Mutex<Broadcaster>>,
}

impl Server {
    /*
       Crea un nuevo servidor de sockets TCP.
    */
    pub fn new(port: u16) -> Result<Self, std::io::Error> {
        let socket_address = SocketAddr::from(([0, 0, 0, 0], port));
        let listener = TcpListener::bind(socket_address)?;

        let state = Arc::new(Mutex::new(ServerState::new()));
        let broadcaster = Arc::new(Mutex::new(Broadcaster::new()));

        Ok(Self {
            listener,
            state,
            broadcaster,
        })
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
                            let id = self.state.lock().unwrap().get_next_id();
                            let reader = BufReader::new(socket_clone);
                            let writer = BufWriter::new(socket);
                            let (sender, receiver) = mpsc::channel::<ClientMessage>();
                            let state = Arc::clone(&self.state);

                            // TODO: incorporar a bitácora
                            println!("Conexion aceptada");
                            println!("\tCliente: {:?}", reader.get_ref().peer_addr());
                            println!("\tPuerto: {:?}", reader.get_ref().peer_addr()?.port());

                            self.broadcaster
                                .lock()
                                .unwrap()
                                .add_client(id, sender.clone());

                            // Manejar mensajes desde el cliente
                            let handler = ClientHandler::new(id, sender, state);
                            thread::spawn(|| handle_input_from_client(reader, handler));

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
