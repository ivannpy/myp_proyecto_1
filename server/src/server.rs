use crate::handlers::ClientHandler;
use crate::model::broadcaster::Broadcaster;
use crate::model::server_state::ServerState;
use crate::network_connection::{ServerNetworkReader, ServerNetworkWriter};
use protocol::messages::client_message::ClientMessage;
use std::io::{BufReader, BufWriter};
use std::net::{SocketAddr, TcpListener};
use std::sync::{Arc, Mutex, mpsc};
use std::thread;

/// Servidor de sockets TCP
///
/// - `listener`: El socket de escucha del servidor
/// - `state`: El estado del servidor
/// - `broadcaster`: El mensajeador del servidor
pub struct Server {
    listener: TcpListener,
    state: Arc<Mutex<ServerState>>,
    broadcaster: Arc<Mutex<Broadcaster>>,
}

impl Server {
    /// Crea un nuevo servidor de sockets en el puerto dado.
    ///
    /// - `port`: El puerto en el que se va a iniciar el servidor
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

    /// Levanta el servidor de sockets TCP
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
                                Ok(mut broadcaster) => broadcaster.add_client(id, sender.clone()),
                                Err(_) => {
                                    eprintln!("Error al agregar cliente al broadcaster");
                                    continue;
                                }
                            };

                            // Manejar mensajes desde el cliente
                            let handler = ClientHandler::new(id, state, broadcaster);
                            let mut server_reader = ServerNetworkReader::new(reader, handler);
                            thread::spawn(move || server_reader.handle_input_from_client());

                            // Manejar mensajes hacia el cliente
                            let mut server_writer = ServerNetworkWriter::new(writer, receiver);
                            thread::spawn(move || server_writer.handle_output_to_client());
                        }
                        Err(e) => {
                            println!("Error clonando socket: {}", e);
                            continue;
                        }
                    }
                }
                _ => {
                    println!("Error al aceptar conexión");
                    continue;
                }
            }
        }
    }
}
