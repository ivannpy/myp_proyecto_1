use crate::broadcaster::Broadcaster;
use crate::model::room::Room;
use crate::model::server_state::ServerState;
use crate::model::user::User;
use protocol::messages::client_message::ClientMessage;
use protocol::messages::responses::{Operation, Result};
use protocol::messages::server_message::ServerMessage;
use protocol::status::user::UserStatus;
use std::collections::HashSet;
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
    state: Arc<Mutex<ServerState>>,
    broadcaster: Arc<Mutex<Broadcaster>>,
}

impl ClientHandler {
    ///
    /// Crea un nuevo manejador del cliente
    ///
    pub fn new(
        id: usize,
        state: Arc<Mutex<ServerState>>,
        broadcaster: Arc<Mutex<Broadcaster>>,
    ) -> Self {
        Self {
            username: None,
            id,
            state,
            broadcaster,
        }
    }

    ///
    /// Responde al cliente
    ///
    fn reply_to_client(&self, reply: &ClientMessage) {
        match self.broadcaster.lock() {
            Ok(mut b) => {
                let _ = b.send_message_to(&self.id, reply);
            }
            Err(_) => {}
        }
    }

    ///
    /// Envía mensaje a todos los demás usuarios
    ///
    fn alert_to_others(&self, alert: &ClientMessage) {
        match self.broadcaster.lock() {
            Ok(mut b) => {
                let _ = b.send_message_to_all_except(&self.id, alert);
            }
            Err(_) => {}
        }
    }

    ///
    /// Envía un mensaje a todos los usuarios de un cuarto
    ///
    fn send_to_room(&self, roomname: &str, msg: &ClientMessage) {
        let mut ids_in_room = Vec::new();
        match self.state.lock() {
            Ok(mut state) => {
                if let Some(room) = state.get_rooms().get(roomname) {
                    for (id, _) in room.get_users() {
                        ids_in_room.push(id);
                    }
                    match self.broadcaster.lock() {
                        Ok(mut b) => {
                            let _ = b.send_message_to_room(ids_in_room, msg);
                        }
                        Err(_) => {}
                    }
                }
            }
            Err(_) => {}
        }
    }

    ///
    /// Envía un mensaje a todos los usuarios dados
    ///
    fn send_to_users(&self, usernames: Vec<&str>, msg: &ClientMessage) {
        let mut ids_to_send = Vec::new();

        match self.state.lock() {
            Ok(mut state) => {
                let users = state.get_users();
                for username in usernames {
                    if let Some(user) = users.get(username) {
                        ids_to_send.push(user.get_id());
                    }
                }
                match self.broadcaster.lock() {
                    Ok(mut b) => {
                        let _ = b.send_message_to_room(ids_to_send, msg);
                    }
                    Err(_) => {}
                }
            }
            Err(_) => {}
        }
    }

    ///
    /// Revisa que el cliente se haya identificado
    ///
    fn check_username(&self) -> Option<String> {
        if let Some(username) = &self.username {
            Some(username.clone())
        } else {
            let reply = ClientMessage::Response {
                operation: Operation::Invalid,
                result: Result::NotIdentified,
                extra: None,
            };
            self.reply_to_client(&reply);
            None
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
    ///
    ///
    fn handle_identify(&mut self, username: String) {
        let reply: ClientMessage;

        match self.state.lock() {
            Ok(mut state) => {
                if state.get_users().contains_key(&username) {
                    reply = ClientMessage::Response {
                        operation: Operation::Identify,
                        result: Result::UserAlreadyExists,
                        extra: Some(username.clone()),
                    }
                } else {
                    let user = User::new(
                        username.clone(),
                        UserStatus::Active,
                        self.id,
                        HashSet::new(),
                        HashSet::new(),
                    );
                    println!("User {} inserted with id {}", username, user.get_id());

                    state.insert_user(user);
                    self.username = Some(username.clone());

                    reply = ClientMessage::Response {
                        operation: Operation::Identify,
                        result: Result::Success,
                        extra: Some(username.clone()),
                    };

                    // A cada cliente enviarle NEW_USER
                    let alert = ClientMessage::NewUser {
                        username: username.clone(),
                    };

                    self.alert_to_others(&alert);
                }
                self.reply_to_client(&reply);
            }
            Err(_) => {}
        }
    }

    ///
    /// Maneja user status
    ///
    ///
    ///
    fn handle_status(&mut self, new_status: UserStatus) {
        let username = match self.check_username() {
            Some(username) => username,
            None => return,
        };

        // Cambia el estado del usuario.

        let result = self
            .state
            .lock()
            .unwrap()
            .change_user_status(&username, new_status.clone());

        // Avisar a todos los demás
        let alert = ClientMessage::NewStatus {
            username,
            status: new_status,
        };

        self.alert_to_others(&alert);
    }

    ///
    /// Maneja users
    ///
    ///
    ///
    fn handle_users(&mut self) {
        let username = match self.check_username() {
            Some(username) => username,
            None => return,
        };

        // Respuesta con los status de los usuarios.
        let locked_state = self.state.lock().unwrap();
        let reply = ClientMessage::UserList {
            users: locked_state.get_users_status(),
        };
        self.reply_to_client(&reply);
    }

    ///
    /// Maneja text
    ///
    /// username_to es el nombre del usuario que debe recibir el mensaje.
    ///
    /// TODO: corregir posible deadlock
    fn handle_text(&mut self, username_to: String, text: String) {
        let username = match self.check_username() {
            Some(username) => username,
            None => return,
        };

        let username_from = username;

        let reply;

        let locked_state = self.state.lock().unwrap();

        let users = locked_state.get_users();
        if users.contains_key(&username_to) {
            reply = ClientMessage::TextFrom {
                username: username_from,
                text: text.clone(),
            };
            let user_id_to = users.get(&username_to).unwrap().get_id();
            let result = self
                .broadcaster
                .lock()
                .unwrap()
                .send_message_to(&user_id_to, &reply);
            if result.is_err() {
                println!("No se pudo enviar el mensaje a {}", self.id);
            }
        } else {
            reply = ClientMessage::Response {
                operation: Operation::Text,
                result: Result::NoSuchUser,
                extra: Some(username_to.clone()),
            };
            let result = self
                .broadcaster
                .lock()
                .unwrap()
                .send_message_to(&self.id, &reply);
            if result.is_err() {
                println!("No se pudo enviar el mensaje a {}", self.id);
            }
        }
    }

    ///
    /// Maneja public text
    ///
    fn handle_public_text(&mut self, text: String) {
        let username = match self.check_username() {
            Some(username) => username,
            None => return,
        };

        let reply = ClientMessage::PublicTextFrom {
            username,
            text: text.clone(),
        };
        self.alert_to_others(&reply);
    }

    ///
    /// Maneja new room
    ///
    fn handle_new_room(&mut self, roomname: String) {
        let username = match self.check_username() {
            Some(username) => username,
            None => return,
        };

        let reply: ClientMessage;

        // Crear el cuarto
        match self.state.lock() {
            Ok(mut state) => {
                let mut rooms = state.get_rooms();
                // Si el cuarto ya existe
                if rooms.contains_key(&roomname) {
                    reply = ClientMessage::Response {
                        operation: Operation::NewRoom,
                        result: Result::RoomAlreadyExists,
                        extra: Some(roomname.clone()),
                    };
                } else {
                    // Si no existe, crearlo
                    let mut room = Room::new(roomname.clone());
                    // Quien lo crea, está en el cuarto
                    room.add_user(&username, self.id);
                    rooms.insert(roomname.clone(), room);
                    reply = ClientMessage::Response {
                        operation: Operation::NewRoom,
                        result: Result::Success,
                        extra: Some(roomname.clone()),
                    };
                }
                self.reply_to_client(&reply);
            }
            Err(_) => {}
        }

    }

    fn verify_is_in_room(&self, username: &str, roomname: &str) -> bool {
        match self.state.lock() {
            Ok(mut state) => {
                let rooms = state.get_rooms();
                if let Some(room) = rooms.get(roomname) {
                    room.is_in(&username)
                } else {
                    false
                }
            }
            Err(_) => {false},
        }
    }

    fn verify_room_exists(&self, roomname: &str) -> bool {
        match self.state.lock() {
            Ok(mut state) => {
                state.get_rooms().contains_key(roomname)
            }
            Err(_) => {false},
        }
    }

    ///
    /// Maneja invite
    ///
    fn handle_invite(&mut self, roomname: String, usernames: Vec<String>) {
        let username = match self.check_username() {
            Some(username) => username,
            None => return,
        };

        // Verificar que el usuario que invita esté en el cuarto.
        let is_in_room = self.verify_is_in_room(&username, &roomname);

        // Verificar que el cuarto y todos los usuarios existan
        let room_exist = self.verify_room_exists(&roomname);

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
                            username,
                            roomname: roomname.clone(),
                        };
                        let result = self
                            .broadcaster
                            .lock()
                            .unwrap()
                            .send_message_to(&user.get_id(), &reply);
                        if result.is_err() {
                            // avisar que no se pudo
                        }
                    }
                    None => {
                        reply = ClientMessage::Response {
                            operation: Operation::Invite,
                            result: Result::NoSuchUser,
                            extra: Some(username),
                        };
                        self.reply_to_client(&reply);
                    }
                }
            }
        }
    }

    ///
    /// Maneja join_room
    ///
    fn handle_join_room(&mut self, roomname: String) {
        let username = match self.check_username() {
            Some(username) => username,
            None => return,
        };

        // Verificar que el cuarto existe
        let room_exists = true;

        // Verificar que este usuario haya sido invitado al cuarto roomname
        let is_invited = true;

        let mut reply: ClientMessage;

        if room_exists && is_invited {
            reply = ClientMessage::Response {
                operation: Operation::JoinRoom,
                result: Result::Success,
                extra: Some(roomname.clone()),
            };
            self.reply_to_client(&reply);

            let alert = ClientMessage::JoinedRoom {
                roomname: roomname.clone(),
                username,
            };
            self.alert_to_others(&alert);
        }
        if !room_exists {
            reply = ClientMessage::Response {
                operation: Operation::JoinRoom,
                result: Result::NoSuchRoom,
                extra: Some(roomname.clone()),
            };
            self.reply_to_client(&reply);
        }
        if !is_invited {
            reply = ClientMessage::Response {
                operation: Operation::JoinRoom,
                result: Result::NotInvited,
                extra: Some(roomname.clone()),
            };

            self.reply_to_client(&reply);
        }
    }

    ///
    /// Maneja room_users
    ///
    fn handle_room_users(&mut self, roomname: String) {
        let username = match self.check_username() {
            Some(username) => username,
            None => return,
        };

        let mut locked_state = self.state.lock().unwrap();
        let rooms = locked_state.get_rooms();

        let room_exists;
        let user_in_room;
        let room = rooms.get(&roomname);

        match room {
            Some(r) => {
                room_exists = true;
                user_in_room = r.is_in(&username);
            }
            None => {
                room_exists = false;
                user_in_room = false;
            }
        }

        let mut reply: ClientMessage;

        if room_exists && user_in_room {
            let users_status = locked_state.get_users_status();
            reply = ClientMessage::RoomUserList {
                roomname: roomname.clone(),
                users: users_status,
            };
            self.reply_to_client(&reply);
        }
        if !room_exists {
            reply = ClientMessage::Response {
                operation: Operation::RoomUsers,
                result: Result::NoSuchRoom,
                extra: Some(roomname.clone()),
            };
            self.reply_to_client(&reply);
        }
        if room_exists && !user_in_room {
            reply = ClientMessage::Response {
                operation: Operation::RoomUsers,
                result: Result::NotJoined,
                extra: Some(roomname.clone()),
            };
            self.reply_to_client(&reply);
        }
    }

    ///
    /// Maneja room text
    fn handle_room_text(&mut self, roomname: String, text: String) {
        let username = match self.check_username() {
            Some(username) => username,
            None => return,
        };

        let mut locked_state = self.state.lock().unwrap();
        let reply;

        if let Some(room) = locked_state.get_rooms().get(&roomname) {
            let user_in_room = room.is_in(&username);

            if user_in_room {
                reply = ClientMessage::RoomTextFrom {
                    roomname: roomname.clone(),
                    username,
                    text: text.clone(),
                };
                // Los ids de los que están en ese cuarto
                let ids = Vec::new();
                self.broadcaster
                    .lock()
                    .unwrap()
                    .send_message_to_room(ids, &reply);
            } else {
                reply = ClientMessage::Response {
                    operation: Operation::RoomText,
                    result: Result::NotJoined,
                    extra: Some(roomname.clone()),
                };
                self.reply_to_client(&reply);
            }
        } else {
            reply = ClientMessage::Response {
                operation: Operation::RoomText,
                result: Result::NoSuchRoom,
                extra: Some(roomname.clone()),
            };
            self.reply_to_client(&reply);
        }
    }

    ///
    /// Maneja leave room
    fn handle_leave_room(&mut self, roomname: String) {
        let username = match self.check_username() {
            Some(username) => username,
            None => return,
        };

        let mut reply;

        {
            let mut locked_state = self.state.lock().unwrap();

            if let Some(room) = locked_state.get_rooms().get_mut(&roomname) {
                if room.is_in(&username) {
                    room.remove_user(&self.id);
                } else {
                    reply = ClientMessage::Response {
                        operation: Operation::LeaveRoom,
                        result: Result::NotJoined,
                        extra: Some(roomname.clone()),
                    };
                    self.reply_to_client(&reply);
                    return;
                }
            } else {
                reply = ClientMessage::Response {
                    operation: Operation::LeaveRoom,
                    result: Result::NoSuchRoom,
                    extra: Some(roomname.clone()),
                };
                self.reply_to_client(&reply);
                return;
            }
        }

        reply = ClientMessage::LeftRoom {
            roomname: roomname.clone(),
            username,
        };

        let ids = Vec::new();
        let _ = self
            .broadcaster
            .lock()
            .unwrap()
            .send_message_to_room(ids, &reply);
    }

    ///
    /// Maneja disconnect
    ///
    fn handle_disconnect(&mut self) {
        let username = match self.check_username() {
            Some(username) => username,
            None => return,
        };

        //let mut left_rooms = Vec::new();

        // eliminar canal con ese cliente
        let mut locked_broadcaster = self.broadcaster.lock().unwrap();
        locked_broadcaster.remove_client(self.id);
        let reply = ClientMessage::Disconnected {
            username: username.clone(),
        };

        // Eliminarlo de todos los cuartos
        let mut locked_state = self.state.lock().unwrap();
        let rooms = locked_state.get_rooms();
        for room in rooms.values_mut() {
            let removed = room.remove_user(&self.id);
            if removed {
                let reply = ClientMessage::LeftRoom {
                    roomname: room.get_room_name(),
                    username: username.clone(),
                };
                let ids = Vec::new();
                locked_broadcaster.send_message_to_room(ids, &reply);
            }
        }

        locked_state.remove_user(&username);

        // Avisar a todos
        self.alert_to_others(&reply);
    }
}

#[cfg(test)]
mod tests {}
