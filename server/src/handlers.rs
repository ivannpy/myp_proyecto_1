use crate::model::server_state::ServerState;
use crate::model::user::User;
use protocol::messages::client_message::ClientMessage;
use protocol::messages::responses::ResponseResult;
use protocol::messages::server_message::ServerMessage;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::net::TcpStream;
use std::sync::{Arc, Mutex, mpsc};

/*
   Maneja la entrada de mensajes desde el cliente.
*/
pub fn handle_input_from_client(mut reader: BufReader<TcpStream>, mut handler: ClientHandler) {
    let mut line = String::new();
    loop {
        line.clear();

        match reader.read_line(&mut line) {
            Ok(0) => break,
            Ok(_) => {
                // Manejar mensaje recibido
                let msg_str = line.trim();

                // Parsear linea a ServerMessage
                let parsed = serde_json::from_str::<ServerMessage>(msg_str);
                match parsed {
                    Ok(server_msg) => {
                        handler.handle_message(server_msg);
                        println!("<<< {}", msg_str);
                    }
                    Err(e) => println!("Error parseando de texto a ServerMessage: {}", e),
                }
            }
            Err(e) => {
                eprintln!(
                    "Error leyendo de {:?}: {}",
                    reader.get_ref().peer_addr().unwrap(),
                    e
                );
            }
        }
    }
}

/*
   Maneja la salida de mensajes al cliente.
*/
pub fn handle_output_to_client(
    mut writer: BufWriter<TcpStream>,
    receiver: mpsc::Receiver<ClientMessage>,
) {
    while let Ok(client_msg) = receiver.recv() {
        let client_msg_str = serde_json::to_string(&client_msg);
        match client_msg_str {
            Ok(mut msg_str) => {
                msg_str.push('\n');
                println!(">>> {:?}", msg_str);
                if writer.write_all(msg_str.as_bytes()).is_err() {
                    break;
                }
                if writer.flush().is_err() {
                    break;
                }
            }
            Err(e) => {
                eprintln!("Error serializando ClientMessage: {}", e);
                continue;
            }
        }
    }
}

// Manejadores del protocolo para comunicarse con los clientes

pub struct ClientHandler {
    username: Option<String>,
    id: usize,
    sender: mpsc::Sender<ClientMessage>,
    state: Arc<Mutex<ServerState>>,
}

impl ClientHandler {
    pub fn new(
        id: usize,
        sender: mpsc::Sender<ClientMessage>,
        state: Arc<Mutex<ServerState>>,
    ) -> Self {
        Self {
            username: None,
            id,
            sender,
            state,
        }
    }

    /*
       Manejador de mensajes del protocolo.
    */
    pub fn handle_message(&mut self, msg: ServerMessage) {
        match msg {
            ServerMessage::Identify { username } => self.handle_identify(username),
        }
    }

    /*
       Maneja la identificación de un usuario.
    */
    fn handle_identify(&mut self, username: String) {
        let reply: ClientMessage;

        {
            let mut locked_state = self.state.lock().unwrap();
            if locked_state.get_users().contains_key(&username) {
                reply = ClientMessage::Response {
                    operation: "IDENTIFY".to_string(),
                    result: ResponseResult::UserAlreadyExists,
                    extra: username.clone(),
                }
            } else {
                let user = User {
                    id: self.id,
                    sender: self.sender.clone(),
                    username: username.clone(),
                };

                println!(
                    "User {} inserted with id {} and sender {:?}",
                    username, user.id, user.sender
                );

                locked_state.insert_user(user);
                self.username = Some(username.clone());

                reply = ClientMessage::Response {
                    operation: "IDENTIFY".to_string(),
                    result: ResponseResult::Success,
                    extra: username.clone(),
                };
            }
        }
        self.sender.send(reply).unwrap();
    }
}
