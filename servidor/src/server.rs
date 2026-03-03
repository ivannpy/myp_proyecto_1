use std::collections::HashMap;
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::thread;
use std::io::{Read, Write};
use std::sync::mpsc;

use crate::utils::parse_msg_to_json;
use crate::connection::Connection;


pub struct Server {
    listener: TcpListener,
    connections: HashMap<String, Connection>,
}

impl Server {
    pub fn new(address: [u8; 4], port: u16) -> Self {
        let socket_address: SocketAddr = SocketAddr::from((address, port));
        let listener: TcpListener = TcpListener::bind(&socket_address).unwrap();
        let connections: HashMap<String, Connection> = HashMap::new();
        Self {
            listener,
            connections,
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
                    let id_conn = socket.peer_addr()?.to_string();
                    let (sender, _) = mpsc::channel::<String>();

                    self.connections.insert(id_conn, Connection::new(sender));

                    // El hilo debe poseer el socket
                    thread::spawn(move || Self::handle_connection(socket));
                }
                Err(e) => {
                    eprintln!("Error aceptando conexión: {}", e)
                },
            }
        }

        Ok(())

    }

    /*
        Maneja una conexion TCP entrante con ECHO.
     */
    fn handle_connection(mut socket: TcpStream) {
        let socket_addr: SocketAddr = socket.peer_addr().ok().unwrap();

        println!("Conexion aceptada");
        println!("Cliente: {:?} puerto: {:?}", socket_addr.ip(), socket_addr.port());

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

                    println!("El cliente {:?} mandó: {:?}", socket_addr, data);
                    socket.write_all(message.as_bytes()).unwrap();
                    println!("El servidor respondió {:?}", message)
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