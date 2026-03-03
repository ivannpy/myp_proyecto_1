use std::collections::HashMap;
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::thread;
use std::io::{Read, Write};
use std::sync::{mpsc, Arc, Mutex};

use crate::utils::parse_msg_to_json;
use crate::connection::Connection;

struct ServerState {
    pub connections: HashMap<String, Connection>,
}

pub struct Server {
    listener: TcpListener,
    state: Arc<Mutex<ServerState>>,
}

impl Server {
    pub fn new(address: [u8; 4], port: u16) -> Self {
        let socket_address: SocketAddr = SocketAddr::from((address, port));
        let listener: TcpListener = TcpListener::bind(&socket_address).unwrap();

        let state = Arc::new(Mutex::new(ServerState {
            connections: HashMap::new(),
        }));

        Self {
            listener,
            state,
        }
    }
    
    /*
        Levanta el servidor TCP con sockets.
     */
    pub fn start(&mut self) -> std::io::Result<()> {
        
        println!("Servidor {} escuchando en el puerto {}",
                 self.listener.local_addr()?.ip(),
                 self.listener.local_addr()?.port());

        for incoming in self.listener.incoming() {
            match incoming {
                Ok(socket) => {
                    // El hilo debe poseer el socket
                    let state = Arc::clone(&self.state);
                    thread::spawn(move || Self::handle_connection(socket, state));
                }
                Err(e) => {
                    eprintln!("Error aceptando conexión: {}", e)
                },
            }
        }

        Ok(())

    }


    fn handle_action(state: &Arc<Mutex<ServerState>>,
                     data: &HashMap<String, String>,
                     conn: &Connection) {
        let msg_type = data.get("type");
        match msg_type {
            Some(msg_type) => {
                let msg_type = msg_type.as_str();
                match msg_type {
                    "IDENTIFY" => {
                        let username = data.get("username").unwrap().clone();

                        {
                            let mut locked = state.lock().unwrap();
                            locked.connections.insert(username.clone(), conn.clone());
                        }

                        let reply = format!(
                            "Conexión establecida con nombre de usuario: {}\n",
                            username
                        );
                        conn.send(reply);
                    },
                    _ => {

                    }
                }
            },
            None => {},
        }

    }

    /*
        Maneja una conexion TCP entrante con ECHO.
     */
    fn handle_connection(mut socket: TcpStream,
                         state: Arc<Mutex<ServerState>>) {
        let socket_addr: SocketAddr = socket.peer_addr().ok().unwrap();

        println!("Conexion aceptada");
        println!("Cliente: {:?} puerto: {:?}", socket_addr.ip(), socket_addr.port());

        let (tx, rx) = mpsc::channel::<String>();
        let conn = Connection::new(tx);

        let mut write_socket = socket.try_clone().unwrap();
        thread::spawn(move || {
            while let Ok(msg) = rx.recv() {
                if write_socket.write_all(msg.as_bytes()).is_err() {
                    break;
                }
            }
        });

        let mut buffer: [u8; 1024] = [0u8; 1024];

        loop {
            match socket.read(&mut buffer) {
                Ok(0) => {
                    // Cerrar conexion
                    println!("Conexion cerrada");
                    break;
                },
                Ok(n) => {
                    let message = String::from_utf8_lossy(&buffer[..n]);
                    let data = parse_msg_to_json(&message);
                    Self::handle_action(&state, &data, &conn);
                },
                Err(e) => {
                    // cerrar conexion
                    eprintln!("Error leyendo de {:?}: {}", socket_addr, e);
                    break;
                },
            }
        }
    }
}