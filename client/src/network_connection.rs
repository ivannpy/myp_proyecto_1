use crate::client_error::ClientError;
use protocol::messages::client_message::ClientMessage;
use protocol::messages::server_message::ServerMessage;
use serde_json;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::net::TcpStream;

/// Envuelve un socket TCP para enviar mensajes al servidor
///
/// - `writer`: Socket TCP envuelto en un  buffer para escritura al socket remoto en el servidor.
pub struct NetworkWriter {
    writer: BufWriter<TcpStream>,
}

impl NetworkWriter {
    /// Envía un mensaje al servidor
    ///
    /// - `message`: Mensaje a enviar al servidor
    pub fn send_message(&mut self, message: &ServerMessage) -> Result<(), ClientError> {
        let msg = serde_json::to_string::<ServerMessage>(message);
        match msg {
            Ok(mut msg) => {
                msg.push('\n');
                msg.push('\0');
                match self.writer.write_all(msg.as_bytes()) {
                    Ok(_) => {
                        self.writer
                            .flush()
                            .map_err(|_| ClientError::ConnectionError)?;
                        Ok(())
                    }
                    Err(_) => Err(ClientError::ConnectionError),
                }
            }
            Err(_) => Err(ClientError::ConnectionError),
        }
    }
}

/// Envuelve un socket TCP para recibir mensajes del servidor
///
/// - `reader`: Socket TCP envuelto en un buffer para lectura del socket remoto en el servidor.
pub struct NetworkReader {
    reader: BufReader<TcpStream>,
}

impl NetworkReader {
    /// Recibe mensajes del servidor
    pub fn receive_message(&mut self) -> Result<ClientMessage, ClientError> {
        let mut line = String::new();
        match self.reader.read_line(&mut line) {
            Ok(0) => Err(ClientError::ConnectionError),
            Ok(_) => serde_json::from_str::<ClientMessage>(line.trim().trim_matches('\0'))
                .map_err(|_| ClientError::ConnectionError),
            Err(_) => Err(ClientError::ConnectionError),
        }
    }
}

/// Crea una conexión con el servidor
/// Regresa un socket TCP para enviar y recibir mensajes
///
/// - `address`: Dirección del servidor
/// - `port`: Puerto del servidor
pub fn connect(address: &str, port: u16) -> Result<(NetworkReader, NetworkWriter), ClientError> {
    let socket = TcpStream::connect((address, port)).map_err(|_| ClientError::ConnectionError)?;
    let socket_clone = socket
        .try_clone()
        .map_err(|_| ClientError::ConnectionError)?;
    Ok((
        NetworkReader {
            reader: BufReader::new(socket),
        },
        NetworkWriter {
            writer: BufWriter::new(socket_clone),
        },
    ))
}
