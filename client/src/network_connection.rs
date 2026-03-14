use protocol::messages::client_message::ClientMessage;
use protocol::messages::server_message::ServerMessage;
use serde_json;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::net::TcpStream;

struct NetworkConnection {
    reader: BufReader<TcpStream>,
    writer: BufWriter<TcpStream>,
}

impl NetworkConnection {
    fn new(address: &str, port: u16) {
        let socket_in = TcpStream::connect((address, port)).unwrap();
        let socket_out = socket_in.try_clone().unwrap();

        let reader = BufReader::new(socket_in);
        let writer = BufWriter::new(socket_out);
        Self { reader, writer };
    }

    pub fn send_message(&mut self, message: &ServerMessage) {
        let mut msg = serde_json::to_string::<ServerMessage>(message).unwrap();
        msg.push('\n');
        self.writer.write_all(msg.as_bytes()).unwrap();
    }

    pub fn receive_message(&mut self) -> ClientMessage {
        let mut line = String::new();
        self.reader.read_line(&mut line).unwrap();
        serde_json::from_str::<ClientMessage>(&line).unwrap()
    }
}
