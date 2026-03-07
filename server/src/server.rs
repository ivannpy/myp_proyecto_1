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

    /*
        Maneja una accion de un cliente.
     */
    fn handle_action(state: &Arc<Mutex<ServerState>>,
                     data: &HashMap<String, String>,
                     conn: &Connection) {
        let msg_type = data.get("type");
        match msg_type {
            Some(msg_type) => {
                let msg_type = msg_type.as_str();
                match msg_type {
                    "IDENTIFY" => {
                        let reply: String;
                        let username = data.get("username").unwrap().clone();

                        {
                            let mut locked = state.lock().unwrap();
                            if locked.connections.contains_key(&username) {
                                let mut reply_hashmap = HashMap::new();
                                reply_hashmap.insert("type".to_string(), "RESPONSE".to_string());
                                reply_hashmap.insert("operation".to_string(), "IDENTIFY".to_string());
                                reply_hashmap.insert("result".to_string(), "USER_ALREADY_EXISTS".to_string());
                                reply_hashmap.insert("extra".to_string(), username.clone());

                                reply = serde_json::to_string(&reply_hashmap).unwrap();
                            } else{
                                locked.connections.insert(username.clone(), conn.clone());
                                let mut reply_hashmap = HashMap::new();
                                reply_hashmap.insert("type".to_string(), "RESPONSE".to_string());
                                reply_hashmap.insert("operation".to_string(), "IDENTIFY".to_string());
                                reply_hashmap.insert("result".to_string(), "SUCCESS".to_string());
                                reply_hashmap.insert("extra".to_string(), username.clone());
                                reply = serde_json::to_string(&reply_hashmap).unwrap();
                            }
                        }

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
        Maneja una conexion TCP entrante
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
            while let Ok(mut msg) = rx.recv() {
                msg.push('\n');
                println!(">>> {}", msg);
                if write_socket.write_all(msg.as_bytes()).is_err() {
                    break;
                }
            }
        });

        let mut buffer: [u8; 1024] = [0u8; 1024];

        loop {
            // TODO: poder identificar que socket se cerró
            match socket.read(&mut buffer) {
                Ok(0) => {
                    // Cerrar conexion
                    println!("Conexion cerrada");
                    break;
                },
                Ok(n) => {
                    let message = String::from_utf8_lossy(&buffer[..n]);
                    let data = parse_msg_to_json(&message);
                    println!("<<< {}", message);
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