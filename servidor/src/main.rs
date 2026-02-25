use std::io::{Read, Write};
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::thread;


/*
    Maneja una conexion TCP entrante con ECHO.
 */
fn maneja_conexion(mut conexion: TcpStream) {
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


/*
    Levanta el servidor TCP con sockets.
 */
fn levantar_servidor() -> std::io::Result<()> {
    let direccion_socket: SocketAddr = SocketAddr::from(([127, 0, 0, 1], 7878));
    let escucha: TcpListener = TcpListener::bind(&direccion_socket)?;

    println!("Servidor {} escuchando en el puerto {}",
             escucha.local_addr()?.ip(),
             escucha.local_addr()?.port());

    for conexion in escucha.incoming() {
        match conexion {
            Ok(conexion) => {
                thread::spawn(|| maneja_conexion(conexion));
            }
            Err(e) => {
                eprintln!("Error aceptando conexión: {}", e)
            },
        }
    }

    Ok(())

}

/*
    Punto de entrada del programa.
 */
fn main() {
    levantar_servidor().unwrap();
}