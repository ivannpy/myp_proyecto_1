use crate::broadcaster::Broadcaster;
use crate::model::server_state::ServerState;
use crate::model::user::User;
use protocol::messages::client_message::ClientMessage;
use protocol::messages::responses::{Operation, Result};
use protocol::messages::server_message::ServerMessage;
use protocol::status::user::UserStatus;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::net::TcpStream;
use std::sync::{Arc, Mutex, mpsc};

///
/// Maneja la entrada de mensajes desde el cliente.
///
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

///
/// Maneja la salida de mensajes al cliente.
///
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

///
/// Maneja la comunicación del servidor con un cliente
///
pub struct ClientHandler {
    username: Option<String>,
    id: usize,
    sender: mpsc::Sender<ClientMessage>,
    state: Arc<Mutex<ServerState>>,
    broadcaster: Arc<Mutex<Broadcaster>>,
}

impl ClientHandler {
    ///
    /// Crea un nuevo manejador del cliente
    ///
    pub fn new(
        id: usize,
        sender: mpsc::Sender<ClientMessage>,
        state: Arc<Mutex<ServerState>>,
        broadcaster: Arc<Mutex<Broadcaster>>,
    ) -> Self {
        Self {
            username: None,
            id,
            sender,
            state,
            broadcaster,
        }
    }

    ///
    /// Revisa que el cliente se haya identificado
    ///
    fn check_username(&self) {
        if self.username.is_none() {
            let reply = ClientMessage::Response {
                operation: Operation::Invalid,
                result: Result::NotIdentified,
                extra: None,
            };
            self.sender.send(reply).unwrap();
            // DESCONECTAR
        }
    }

    ///
    /// Maneja los mensajes que recibe el servidor.
    ///
    pub fn handle_message(&mut self, msg: ServerMessage) {
        match msg {
            ServerMessage::Identify { username } => self.handle_identify(username),
            ServerMessage::Status { status } => self.handle_status(status),
            ServerMessage::Users => self.handle_users(),
            ServerMessage::Text { username, text } => self.handle_text(username, text),
            ServerMessage::PublicText { text } => self.handle_public_text(text),
            ServerMessage::NewRoom { roomname } => self.handle_new_room(roomname),
            ServerMessage::Invite {
                roomname,
                usernames,
            } => self.handle_invite(roomname, usernames),
            ServerMessage::JoinRoom { roomname } => self.handle_join_room(roomname),
            ServerMessage::RoomUsers { roomname } => self.handle_room_users(roomname),
            ServerMessage::RoomText { roomname, text } => self.handle_room_text(roomname, text),
            ServerMessage::LeaveRoom { roomname } => self.handle_leave_room(roomname),
            ServerMessage::Disconnect => self.handle_disconnect(),
        }
    }

    ///
    /// Maneja la identificación de un usuario.
    ///
    /// El servidor recibe la petición de identificar a un cliente con el username dado.
    ///
    /// TODO: quitar .unwrap()
    ///
    fn handle_identify(&mut self, username: String) {
        let reply: ClientMessage;

        {
            let mut locked_state = self.state.lock().unwrap();

            if locked_state.get_users().contains_key(&username) {
                reply = ClientMessage::Response {
                    operation: Operation::Identify,
                    result: Result::UserAlreadyExists,
                    extra: Some(username.clone()),
                }
            } else {
                let user = User::new(username.clone(), UserStatus::Active, self.id);

                println!("User {} inserted with id {}", username, user.get_id());

                locked_state.insert_user(user);
                self.username = Some(username.clone());

                reply = ClientMessage::Response {
                    operation: Operation::Identify,
                    result: Result::Success,
                    extra: Some(username.clone()),
                };
            }
        }
        self.sender.send(reply).unwrap();

        // A cada cliente enviarle NEW_USER
        let alert = ClientMessage::NewUser {
            username: username.clone(),
        };

        self.broadcaster.lock().unwrap().send_message_to_all(&alert);
    }

    ///
    /// Maneja user status
    ///
    /// TODO: quitar .unwrap()
    ///
    fn handle_status(&mut self, new_status: UserStatus) {
        self.check_username();
        let username = self.username.clone().unwrap();

        // Cambia el estado del usuario.
        self.state
            .lock()
            .unwrap()
            .change_user_status(&username, new_status.clone());

        // Avisar a todos los demás
        let alert = ClientMessage::NewStatus {
            username: self.username.clone().unwrap(),
            status: new_status,
        };

        self.broadcaster
            .lock()
            .unwrap()
            .send_message_to_all_except(&self.id, &alert);
    }

    ///
    /// Maneja users
    ///
    /// TODO: quitar .unwrap()
    ///
    fn handle_users(&mut self) {
        self.check_username();

        // Respuesta con los status de los usuarios.
        let locked_state = self.state.lock().unwrap();
        let reply = ClientMessage::UserList {
            users: locked_state.get_users_status(),
        };
        self.sender.send(reply).unwrap();
    }

    ///
    /// Maneja text
    ///
    /// username_to es el nombre del usuario que debe recibir el mensaje.
    ///
    ///
    fn handle_text(&mut self, username_to: String, text: String) {
        self.check_username();
        let username_from = self.username.clone().unwrap();

        let mut reply;

        let locked_state = self.state.lock().unwrap();

        let users = locked_state.get_users();
        if users.contains_key(&username_to) {
            reply = ClientMessage::TextFrom {
                username: username_from,
                text: text.clone(),
            }
        } else {
            reply = ClientMessage::Response {
                operation: Operation::Text,
                result: Result::NoSuchUser,
                extra: Some(username_to.clone()),
            };
        }
        self.broadcaster
            .lock()
            .unwrap()
            .send_message_to(self.id, &reply);
    }

    ///
    /// Maneja public text
    ///
    fn handle_public_text(&mut self, text: String) {
        self.check_username();

        let reply = ClientMessage::PublicTextFrom {
            username: self.username.clone().unwrap(),
            text: text.clone(),
        };
        self.broadcaster.lock().unwrap().send_message_to_all(&reply);
    }

    ///
    /// Maneja new room
    ///
    fn handle_new_room(&mut self, roomname: String) {
        self.check_username();

        let reply;

        let created = true;
        // TODO: Crear cuarto en el servidor

        if created {
            reply = ClientMessage::Response {
                operation: Operation::NewRoom,
                result: Result::Success,
                extra: Some(roomname.clone()),
            };
        } else {
            reply = ClientMessage::Response {
                operation: Operation::NewRoom,
                result: Result::RoomAlreadyExists,
                extra: Some(roomname.clone()),
            }
        }
        self.sender.send(reply).unwrap();
    }

    ///
    /// Maneja invite
    ///
    fn handle_invite(&mut self, roomname: String, usernames: Vec<String>) {
        self.check_username();

        // Verificar que el cuarto y todos los usuarios existan
        let room_exist = true;

        let mut reply;
        let locked_state = self.state.lock().unwrap();

        if room_exist {
            let users = locked_state.get_users();
            for username in usernames {
                // Si el usuario ya está en la sala o ya había sido invitado, ignorar
                let user_to = users.get(&username);

                match &user_to {
                    Some(user) => {
                        reply = ClientMessage::Invitation {
                            username: self.username.clone().unwrap(),
                            roomname: roomname.clone(),
                        };
                        let result = self.broadcaster
                            .lock()
                            .unwrap()
                            .send_message_to(user.get_id(), &reply);
                        if result.is_err() {
                            // avisar que no se pudo
                        }
                    }
                    None => {
                        reply = ClientMessage::Response {
                            operation: Operation::Invite,
                            result: Result::NoSuchUser,
                            extra: Some(user_to.unwrap().get_username()),
                        };
                        self.sender.send(reply).unwrap();
                    }
                }
            }
        }
    }

    ///
    /// Maneja join_room
    ///
    fn handle_join_room(&mut self, roomname: String) {
        self.check_username();
    }

    ///
    /// Maneja room_users
    ///
    fn handle_room_users(&mut self, roomname: String) {
        self.check_username();
    }

    ///
    /// Maneja room text
    fn handle_room_text(&mut self, roomname: String, text: String) {
        self.check_username();
    }

    ///
    /// Maneja leave room
    fn handle_leave_room(&mut self, roomname: String) {
        self.check_username();
    }

    ///
    /// Maneja disconnect
    ///
    fn handle_disconnect(&mut self) {
        self.check_username();
    }
}

#[cfg(test)]
mod tests {}
