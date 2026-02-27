use std::net::{SocketAddr, TcpListener, TcpStream};
use std::thread;
use std::io::{Read, Write};

pub struct Server {
    listener: TcpListener,
}

impl Server {
    pub fn new(address: [u8; 4], port: u16) -> Self {
        let socket_address: SocketAddr = SocketAddr::from((address, port));
        let listener: TcpListener = TcpListener::bind(&socket_address).unwrap();
        Server {
            listener,
        }
    }
    
    /*
        Levanta el servidor TCP con sockets.
     */
    pub fn start(&self) -> std::io::Result<()> {
        
        println!("Servidor {} escuchando en el puerto {}",
                 self.listener.local_addr()?.ip(),
                 self.listener.local_addr()?.port());

        for conexion in self.listener.incoming() {
            match conexion {
                Ok(conexion) => {
                    thread::spawn(|| Self::handle_connection(conexion));
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
    fn handle_connection(mut conexion: TcpStream) {
        let direccion_socket: SocketAddr = conexion.peer_addr().ok().unwrap();

        println!("Cliente: {:?} puerto: {:?}", direccion_socket.ip(), direccion_socket.port());

        let mut buffer: [u8; 1024] = [0u8; 1024];

        loop {
            match conexion.read(&mut buffer) {
                Ok(0) => {
                    println!("Conexion cerrada");
                    break;
                },
                Ok(n) => {
                    println!("Leído del cliente: {:?}", buffer);

                    let mensaje = String::from_utf8_lossy(&buffer[..n]);

                    println!("El cliente {:?} mandó: {:?}", direccion_socket, mensaje);

                    conexion.write_all(&buffer[..n]).unwrap();
                    println!("Enviado al cliente: {:?}", buffer);
                    println!("El servidor respondió {:?}", mensaje)
                },
                Err(e) => {
                    eprintln!("Error leyendo de {:?}: {}", direccion_socket, e);
                    break;
                },
            }
        }
    }
}