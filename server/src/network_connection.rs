use crate::handlers::ClientHandler;
use protocol::messages::client_message::ClientMessage;
use protocol::messages::server_message::ServerMessage;
use std::io::{BufRead, BufReader, BufWriter, Write};
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
        let mut line = String::new();
        loop {
            line.clear();

            match self.reader.read_line(&mut line) {
                Ok(0) => break,
                Ok(_) => {
                    let msg_str = line.trim();

                    // Parsear linea a ServerMessage
                    match serde_json::from_str::<ServerMessage>(msg_str) {
                        Ok(msg) => {
                            println!("<<< {}", msg_str);
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
            match serde_json::to_string(&msg) {
                Ok(mut msg_str) => {
                    msg_str.push('\n');
                    println!(">>> {}", msg_str);
                    if self
                        .writer
                        .write_all(msg_str.as_bytes())
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
