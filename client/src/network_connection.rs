use protocol::messages::client_message::ClientMessage;
use protocol::messages::server_message::ServerMessage;
use serde_json;
use std::io::{BufRead, BufReader, BufWriter, Write, ErrorKind};
use std::net::TcpStream;
use crate::client_error::ClientError;

pub struct NetworkWriter {
    writer: BufWriter<TcpStream>,
}

pub struct NetworkReader {
    reader: BufReader<TcpStream>,
}

impl NetworkWriter {
    pub fn send_message(&mut self, message: &ServerMessage) -> Result<(), ClientError> {
        let msg = serde_json::to_string::<ServerMessage>(message);
        match msg {
            Ok(mut msg) => {
                msg.push('\n');
                println!("Mensaje a enviar al servidor {:?}", msg);
                match self.writer.write_all(msg.as_bytes()) {
                    Ok(_) => {
                        self.writer.flush().map_err(|_| ClientError::ConnectionError)?;
                        Ok(())
                    }
                    Err(_) => Err(ClientError::ConnectionError),
                }
            }
            Err(_) => Err(ClientError::ConnectionError),
        }
    }
}

impl NetworkReader {
    pub fn receive_message(&mut self) -> Result<ClientMessage, ClientError> {
        let mut line = String::new();
        match self.reader.read_line(&mut line) {
            Ok(0) => Err(ClientError::ConnectionError), // conexión cerrada
            Ok(_) => {
                serde_json::from_str::<ClientMessage>(&line)
                    .map_err(|_| ClientError::ConnectionError)
            }
            Err(_) => Err(ClientError::ConnectionError),
        }
    }
}

pub fn connect(address: &str, port: u16) -> Result<(NetworkReader, NetworkWriter), ClientError> {
    let socket = TcpStream::connect((address, port)).map_err(|_| ClientError::ConnectionError)?;
    let socket_clone = socket.try_clone().map_err(|_| ClientError::ConnectionError)?;
    Ok((
        NetworkReader { reader: BufReader::new(socket) },
        NetworkWriter { writer: BufWriter::new(socket_clone) },
    ))
}