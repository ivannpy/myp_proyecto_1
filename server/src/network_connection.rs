use crate::handlers::ClientHandler;
use protocol::messages::client_message::ClientMessage;
use protocol::messages::server_message::ServerMessage;
use std::io::{BufReader, BufWriter, Read, Write};
use std::net::TcpStream;
use std::sync::mpsc;

/// Maneja la entrada de mensajes desde el cliente.
///
/// - `reader`: Un buffer con el socket tcp que recibe el mensaje del cliente.
/// - `handler`: El manejador de mensajes del cliente.
pub struct ServerNetworkReader {
    reader: BufReader<TcpStream>,
    handler: ClientHandler,
}

impl ServerNetworkReader {
    /// Construye un nuevo ServerNetworkReader.
    pub fn new(reader: BufReader<TcpStream>, handler: ClientHandler) -> Self {
        Self { reader, handler }
    }

    /// Maneja la entrada de mensajes desde el cliente.
    pub fn handle_input_from_client(&mut self) {
        let mut line_buffer = vec![0; 1024];

        loop {
            line_buffer.clear();
            println!("Esperando mensaje...");

            match self.reader.read(&mut line_buffer) {
                Ok(0) => break,
                Ok(n) => {
                    println!("Mensaje recibido. Longitud {} ", n);
                    // Buscamos el final de la linea ya sea un 0x00 o un \n
                    let end_pos = line_buffer[..n]
                        .iter()
                        .position(|&b| b == b'\0' || b == b'\n')
                        .unwrap_or(n);
                    println!("Mensaje terminado en {}", end_pos);

                    // Leemos hasta el \0 o hasta el \n, lo que ocurra primero
                    let msg_str = String::from_utf8_lossy(&line_buffer[..end_pos])
                        .trim()
                        .to_string();

                    println!("<<< {}", msg_str);

                    match serde_json::from_str::<ServerMessage>(msg_str.as_str()) {
                        Ok(msg) => {
                            println!("Mensaje recibido: {:?}", msg);
                            self.handler.handle_message(msg);
                        }
                        Err(_) => println!("Error parseando mensaje de {}", self.handler.get_id()),
                    }
                }
                Err(e) => {
                    println!(
                        "Error leyendo de {:?}: {}",
                        self.reader.get_ref().peer_addr(),
                        e
                    );
                }
            }
        }
        self.handler.handle_disconnect();
    }
}

/// Maneja la salida de mensajes al cliente.
///
/// - `writer`: Un buffer con el socket tcp que envía mensajes al cliente
/// - `receiver`: El canal de recepción de mensajes del cliente
///               Este objeto tiene acceso al sender del handler.
///               El sender le dice al receiver qué mensaje se debe enviar al cliente.
pub struct ServerNetworkWriter {
    writer: BufWriter<TcpStream>,
    receiver: mpsc::Receiver<ClientMessage>,
}

impl ServerNetworkWriter {
    /// Construye un nuevo ServerNetworkWriter.
    pub fn new(writer: BufWriter<TcpStream>, receiver: mpsc::Receiver<ClientMessage>) -> Self {
        Self { writer, receiver }
    }

    /// Maneja la salida de mensajes al cliente.
    pub fn handle_output_to_client(&mut self) {
        while let Ok(msg) = self.receiver.recv() {
            println!("Enviando mensaje: {:?}", msg);
            match serde_json::to_string(&msg) {
                Ok(mut msg_str) => {
                    msg_str.push('\n');
                    msg_str.push('\0');

                    println!(">>> {}", msg_str);
                    let msg_bytes = msg_str.into_bytes();
                    if self
                        .writer
                        .write_all(&msg_bytes)
                        .and_then(|_| self.writer.flush())
                        .is_err()
                    {
                        match self.writer.get_ref().peer_addr() {
                            Ok(addr) => println!("Error enviando mensaje a {}", addr),
                            Err(e) => println!("Error enviando mensaje: {}", e),
                        }
                        break;
                    }
                }
                Err(e) => {
                    println!("Error serializando mensaje hacia un cliente: {}", e);
                    break;
                }
            }
        }
    }
}
