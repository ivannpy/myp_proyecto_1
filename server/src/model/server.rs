use crate::broadcaster::Broadcaster;
use crate::handlers::{ClientHandler, handle_input_from_client, handle_output_to_client};
use crate::model::server_state::ServerState;
use protocol::messages::client_message::ClientMessage;
use std::io::{BufReader, BufWriter};
use std::net::{SocketAddr, TcpListener};
use std::sync::{Arc, Mutex, mpsc};
use std::thread;

///
/// Servidor de sockets TCP
///
pub struct Server {
    listener: TcpListener,
    state: Arc<Mutex<ServerState>>,
    broadcaster: Arc<Mutex<Broadcaster>>,
}

impl Server {
    ///
    /// Crea un nuevo servidor de sockets
    ///
    pub fn new(port: u16) -> Result<Self, std::io::Error> {
        let socket_address = SocketAddr::from(([0, 0, 0, 0], port));
        let listener = match TcpListener::bind(socket_address) {
            Ok(listener) => listener,
            Err(e) => {
                eprintln!("Error al iniciar el servidor en el puerto: {}", port);
                return Err(e);
            }
        };

        let state = Arc::new(Mutex::new(ServerState::new()));
        let broadcaster = Arc::new(Mutex::new(Broadcaster::new()));

        Ok(Self {
            listener,
            state,
            broadcaster,
        })
    }

    ///
    /// Levanta el servidor de sockets TCP
    ///
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
                            let id = match self.state.lock() {
                                Ok(state) => state.get_next_id(),
                                Err(e) => {
                                    eprintln!("Error al obtener id: {}", e);
                                    continue;
                                }
                            };
                            let reader = BufReader::new(socket_clone);
                            let writer = BufWriter::new(socket);
                            let (sender, receiver) = mpsc::channel::<ClientMessage>();
                            let state = Arc::clone(&self.state);
                            let broadcaster = Arc::clone(&self.broadcaster);

                            // TODO: incorporar a bitácora
                            println!("Conexion aceptada");
                            println!("\tCliente: {:?}", reader.get_ref().peer_addr());
                            println!("\tPuerto: {:?}", reader.get_ref().peer_addr()?.port());

                            let _ = match self.broadcaster.lock() {
                                Ok(mut broadcaster) => broadcaster.add_client(&id, sender.clone()),
                                Err(_) => {
                                    eprintln!("Error al agregar cliente al broadcaster");
                                    continue;
                                }
                            };

                            // Manejar mensajes desde el cliente
                            let handler = ClientHandler::new(id, state, broadcaster);
                            thread::spawn(move || handle_input_from_client(reader, handler));

                            // Manejar mensajes hacia el cliente
                            thread::spawn(move || handle_output_to_client(writer, receiver));
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
