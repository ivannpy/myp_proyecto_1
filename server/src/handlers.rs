use crate::broadcaster::Broadcaster;
use crate::model::room::Room;
use crate::model::server_state::ServerState;
use crate::model::user::User;
use protocol::messages::client_message::ClientMessage;
use protocol::messages::responses::{Operation, Result};
use protocol::messages::server_message::ServerMessage;
use protocol::status::user::UserStatus;
use std::collections::{HashMap, HashSet};
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
    /// Métodos que requieren sincronizar el estado o el broadcaster
    ///

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
    /// Verifica que un usuario esté en un cuarto
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
            Err(_) => false,
        }
    }

    ///
    /// Verifica que un cuarto exista
    ///
    fn verify_room_exists(&self, roomname: &str) -> bool {
        match self.state.lock() {
            Ok(mut state) => state.get_rooms().contains_key(roomname),
            Err(_) => false,
        }
    }

    ///
    /// Verifica si un usuario dado existe en el servidor
    ///
    fn verify_user_exist(&self, usernames: &str) -> bool {
        match self.state.lock() {
            Ok(mut state) => state.get_users().contains_key(usernames),
            Err(_) => false,
        }
    }

    ///
    /// Verifica que un usuario haya sido invitado a un cuarto
    ///
    fn verify_user_invited(&self, username: &str, roomname: &str) -> bool {
        match self.state.lock() {
            Ok(mut state) => {
                if let Some(room) = state.get_rooms().get(roomname) {
                    room.is_invited(&username)
                } else {
                    false
                }
            }
            Err(_) => {false},
        }
    }

    ///
    /// Agrega a un usuario a un cuarto
    ///
    fn add_user_to_room(&mut self, username: &str, roomname: &str) {
        match self.state.lock() {
            Ok(mut state) => {
                if let Some(room) = state.get_rooms().get_mut(roomname) {
                    room.add_user(username, self.id);
                }
            }
            Err(_) => {}
        }
    }

    ///
    /// Elimina a un usuario de un cuarto
    ///
    fn delete_user_from_room(&self, username: &str, roomname: &str) {
        match self.state.lock() {
            Ok(mut state) => {
                if let Some(user) = state.get_users().get(username) {
                    let user_id = user.get_id();

                    if let Some(room) = state.get_rooms().get_mut(roomname) {
                        room.remove_user(&user_id);
                    }
                }
            }
            Err(e) => {}
        }
    }

    ///
    /// Regresa la lista de cuartos donde está un usuario dado
    ///

    fn get_rooms_user_is_in(&self, username: &str) -> Vec<String> {
        let rooms = Vec::new();
        rooms
    }

    ///
    /// Regresa un diccionario con los estados de los usuarios en un cuarto
    ///
    fn get_user_list_room(&self, roomname: &str) -> HashMap<String, UserStatus> {
        let mut room_status = HashMap::new();

        match self.state.lock() {
            Ok(mut state) => {
                if let Some(room) = state.get_rooms().get(roomname) {
                    let users_in_room = room.get_users();
                    let users_status = state.get_users_status();

                    for (_, username) in users_in_room {
                        if let Some(status) = users_status.get(&username) {
                            room_status.insert(username.clone(), status.clone());
                        }
                    }
                }
            }
            Err(_) => {}
        }
        room_status
    }

    ///
    /// Maneja los mensajes que recibe el servidor.
    ///
    pub fn handle_message(&mut self, msg: ServerMessage) {
        match msg {
            ServerMessage::Identify { username } => self.handle_identify(&username),
            ServerMessage::Status { status } => self.handle_status(status),
            ServerMessage::Users => self.handle_users(),
            ServerMessage::Text { username, text } => self.handle_text(&username, &text),
            ServerMessage::PublicText { text } => self.handle_public_text(&text),
            ServerMessage::NewRoom { roomname } => self.handle_new_room(&roomname),
            ServerMessage::Invite {
                roomname,
                usernames,
            } => self.handle_invite(&roomname, usernames),
            ServerMessage::JoinRoom { roomname } => self.handle_join_room(roomname),
            ServerMessage::RoomUsers { roomname } => self.handle_room_users(roomname),
            ServerMessage::RoomText { roomname, text } => self.handle_room_text(roomname, text),
            ServerMessage::LeaveRoom { roomname } => self.handle_leave_room(roomname),
            ServerMessage::Disconnect => self.handle_disconnect(),
        }
    }

    ///
    /// Implementación del protocolo del lado del servidor.
    ///

    ///
    /// Maneja la identificación de un usuario.
    ///
    /// El servidor recibe la petición de identificar a un cliente con el username dado.
    ///
    fn handle_identify(&mut self, username: &str) {
        let reply: ClientMessage;

        // Todo: mover este lock a otra funcion
        match self.state.lock() {
            Ok(mut state) => {
                if state.user_is_online(username) {
                    reply = ClientMessage::Response {
                        operation: Operation::Identify,
                        result: Result::UserAlreadyExists,
                        extra: Some(username.to_string()),
                    };
                    self.reply_to_client(&reply);
                } else {
                    let user = User::new(
                        username.to_string(),
                        UserStatus::Active,
                        self.id,
                        HashSet::new(),
                        HashSet::new(),
                    );

                    state.insert_user(user);
                    self.username = Some(username.to_string());

                    reply = ClientMessage::Response {
                        operation: Operation::Identify,
                        result: Result::Success,
                        extra: Some(username.to_string()),
                    };
                    self.reply_to_client(&reply);

                    // A cada cliente enviarle NEW_USER
                    let alert = ClientMessage::NewUser {
                        username: username.to_string(),
                    };
                    self.alert_to_others(&alert);
                }
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

        // TODO: Mover este lock a otra funcion
        match self.state.lock() {
            Ok(mut status) => match status.change_user_status(&username, new_status.clone()) {
                Ok(_) => {
                    let alert = ClientMessage::NewStatus {
                        username,
                        status: new_status,
                    };
                    // Si cambia exitosamente, alertamos a los demás
                    self.alert_to_others(&alert);
                }
                Err(_) => {}
            },
            Err(_) => {}
        }
    }

    ///
    /// Maneja users
    ///
    fn handle_users(&mut self) {
        let username = match self.check_username() {
            Some(username) => username,
            None => return,
        };

        // Respuesta con los status de los usuarios.
        // TODO: mover este lock a otra funcion
        match self.state.lock() {
            Ok(state) => {
                let reply = ClientMessage::UserList {
                    users: state.get_users_status(),
                };
                self.reply_to_client(&reply);
            }
            Err(_) => {}
        }
    }

    ///
    /// Maneja text
    ///
    /// username_to es el nombre del usuario que debe recibir el mensaje.
    ///
    fn handle_text(&mut self, username_to: &str, text: &str) {
        let username = match self.check_username() {
            Some(username) => username,
            None => return,
        };

        let username_from = username;

        let reply: ClientMessage;

        // TODO: mover este lock a otra funcion

        match self.state.lock() {
            Ok(state) => {
                // Verificar que el destinatario exista en el servidor
                if let Some(_) = state.get_users().get(&username_to.to_string()) {
                    reply = ClientMessage::TextFrom {
                        username: username_from,
                        text: text.to_string(),
                    };
                    // Responder a user_to
                    self.send_to_users(vec![username_to], &reply);
                } else {
                    // El usuario no existe en el servidor
                    reply = ClientMessage::Response {
                        operation: Operation::Text,
                        result: Result::NoSuchUser,
                        extra: Some(username_to.to_string()),
                    };
                    // Responder al cliente
                    self.reply_to_client(&reply);
                }
            }
            Err(_) => {}
        }
    }

    ///
    /// Maneja public text
    ///
    fn handle_public_text(&mut self, text: &str) {
        let username = match self.check_username() {
            Some(username) => username,
            None => return,
        };

        let reply = ClientMessage::PublicTextFrom {
            username,
            text: text.to_string(),
        };
        self.alert_to_others(&reply);
    }

    ///
    /// Maneja new room
    ///
    fn handle_new_room(&mut self, roomname: &str) {
        let username = match self.check_username() {
            Some(username) => username,
            None => return,
        };

        let reply: ClientMessage;

        // Crear el cuarto
        // TODO: mover este lock a otra funcion
        match self.state.lock() {
            Ok(mut state) => {
                let mut rooms = state.get_rooms();
                // Si el cuarto ya existe
                if rooms.contains_key(&roomname.to_string()) {
                    reply = ClientMessage::Response {
                        operation: Operation::NewRoom,
                        result: Result::RoomAlreadyExists,
                        extra: Some(roomname.to_string()),
                    };
                } else {
                    // Si no existe, crearlo
                    let mut room = Room::new(roomname.to_string());
                    // Quien lo crea, está en el cuarto
                    room.add_user(&username, self.id);
                    rooms.insert(roomname.to_string(), room);
                    reply = ClientMessage::Response {
                        operation: Operation::NewRoom,
                        result: Result::Success,
                        extra: Some(roomname.to_string()),
                    };
                }
                self.reply_to_client(&reply);
            }
            Err(_) => {}
        }
    }

    ///
    /// Maneja invite
    ///
    fn handle_invite(&mut self, roomname: &str, usernames: Vec<String>) {
        let username = match self.check_username() {
            Some(username) => username,
            None => return,
        };

        let mut reply: ClientMessage;

        // Si no está en el cuarto no puede invitar
        if !self.verify_is_in_room(&username, &roomname) {
            return;
        }

        // Si el cuarto no existe
        if !self.verify_room_exists(&roomname) {
            reply = ClientMessage::Response {
                operation: Operation::Invite,
                result: Result::NoSuchRoom,
                extra: Some(roomname.to_string()),
            };
            self.reply_to_client(&reply);
        }

        // El cuarto existe

        // Usuarios que no están en el cuarto
        let mut users_to_invite = Vec::new();
        // Usuarios que no exiten
        let mut users_not_exist = Vec::new();

        for user_invited in &usernames {
            if self.verify_is_in_room(user_invited, roomname) {
                continue;
            }
            if !self.verify_user_exist(user_invited) {
                users_not_exist.push(user_invited.to_string())
            } else {
                users_to_invite.push(user_invited.as_str())
            }
        }

        // Notificar de usuarios que no existen
        for user_not_exist in users_not_exist {
            reply = ClientMessage::Response {
                operation: Operation::Invite,
                result: Result::NoSuchUser,
                extra: Some(user_not_exist),
            };
            self.reply_to_client(&reply);
        }

        // Enviar invitación a usuarios existentes que no están en el cuarto
        reply = ClientMessage::Invitation {
            username: username.to_string(),
            roomname: roomname.to_string()
        };

        self.send_to_users(users_to_invite, &reply);

    }

    ///
    /// Maneja join_room
    ///
    fn handle_join_room(&mut self, roomname: String) {
        let username = match self.check_username() {
            Some(username) => username,
            None => return,
        };

        let mut reply: ClientMessage;

        // Verificar que el cuarto existe
        let room_exists = self.verify_room_exists(&roomname);
        if !room_exists {
            reply = ClientMessage::Response {
                operation: Operation::JoinRoom,
                result: Result::NoSuchRoom,
                extra: Some(roomname.to_string()),
            };
            self.reply_to_client(&reply);
            return;
        }

        // El cuarto existe

        // Verificar que este usuario haya sido invitado al cuarto roomname
        let is_invited = self.verify_user_invited(&username, &roomname);

        if !is_invited {
            reply = ClientMessage::Response {
                operation: Operation::JoinRoom,
                result: Result::NotInvited,
                extra: Some(roomname.to_string()),
            };
            self.reply_to_client(&reply);
            return;
        }

        // El usuario fue invitado

        reply = ClientMessage::Response {
            operation: Operation::JoinRoom,
            result: Result::Success,
            extra: Some(roomname.to_string()),
        };
        self.reply_to_client(&reply);

        // Agregar el usuario al cuarto
        self.add_user_to_room(&username, &roomname);

        // Avisar que se unió al cuarto
        let alert = ClientMessage::JoinedRoom {
            username: username.to_string(),
            roomname: roomname.to_string(),
        };
        self.send_to_room(&roomname, &alert)

    }

    ///
    /// Maneja room_users
    ///
    fn handle_room_users(&mut self, roomname: String) {
        let username = match self.check_username() {
            Some(username) => username,
            None => return,
        };

        let mut reply: ClientMessage;

        // Verificar que el cuarto existe
        let room_exists = self.verify_room_exists(&roomname);
        if !room_exists {
            reply = ClientMessage::Response {
                operation: Operation::RoomUsers,
                result: Result::NoSuchRoom,
                extra: Some(roomname.to_string()),
            };
            self.reply_to_client(&reply);
            return;
        }

        // El cuarto existe

        let user_in_room = self.verify_is_in_room(&username, &roomname);

        // El usuario no está en el cuarto
        if !user_in_room {
            reply = ClientMessage::Response {
                operation: Operation::RoomUsers,
                result: Result::NotJoined,
                extra: Some(roomname.to_string()),
            };
            self.reply_to_client(&reply);
            return;
        }

        // El usuario está en el cuarto
        reply = ClientMessage::RoomUserList {
            roomname: roomname.to_string(),
            users: self.get_user_list_room(&roomname),
        };
        self.reply_to_client(&reply);

    }

    ///
    /// Maneja room text
    fn handle_room_text(&mut self, roomname: String, text: String) {
        let username = match self.check_username() {
            Some(username) => username,
            None => return,
        };

        // Verificar que el cuarto existe
        let room_exists = self.verify_room_exists(&roomname);
        if !room_exists {
            let reply = ClientMessage::Response {
                operation: Operation::RoomText,
                result: Result::NoSuchRoom,
                extra: Some(roomname.to_string()),
            };
            self.reply_to_client(&reply);
            return;
        }

        // El cuarto existe

        // Verificar que el usuario este en el cuarto
        let user_in_room = self.verify_is_in_room(&username, &roomname);
        if !user_in_room {
            let reply = ClientMessage::Response {
                operation: Operation::RoomText,
                result: Result::NotJoined,
                extra: Some(roomname.to_string()),
            };
            self.reply_to_client(&reply);
            return;
        }

        // El usuario está en el cuarto

        let alert = ClientMessage::RoomTextFrom {
            roomname: roomname.to_string(),
            username,
            text: text.to_string(),
        };
        self.send_to_room(&roomname, &alert);
    }

    ///
    /// Maneja leave room
    fn handle_leave_room(&mut self, roomname: String) {
        let username = match self.check_username() {
            Some(username) => username,
            None => return,
        };

        let mut reply: ClientMessage;

        // Verificar que el cuarto existe
        let room_exists = self.verify_room_exists(&roomname);
        if !room_exists {
            reply = ClientMessage::Response {
                operation: Operation::LeaveRoom,
                result: Result::NoSuchRoom,
                extra: Some(roomname.to_string()),
            };
            self.reply_to_client(&reply);
            return;
        }

        // El cuarto existe

        // Verificar que el usuario este en el cuarto
        let user_in_room = self.verify_is_in_room(&username, &roomname);
        if !user_in_room {
            reply = ClientMessage::Response {
                operation: Operation::LeaveRoom,
                result: Result::NotJoined,
                extra: Some(roomname.to_string()),
            };
            self.reply_to_client(&reply);
            return;
        }

        // El usuario está en el cuarto
        // Avisar a los demás
        let alert = ClientMessage::LeftRoom {
            roomname: roomname.to_string(),
            username,
        };
        self.send_to_room(&roomname, &alert);

    }

    ///
    /// Maneja disconnect
    ///
    fn handle_disconnect(&mut self) {
        let username = match self.check_username() {
            Some(username) => username,
            None => return,
        };


        // Eliminarlo de la lista de usuarios
        self.state.lock().unwrap().remove_user(&username);

        // Eliminarlo de todos los cuartos donde esté.
        for room in self.state.lock().unwrap().get_rooms().keys() {
            self.delete_user_from_room(&username, room)
        }

        // Avisar que se desconectó
        let alert = ClientMessage::Disconnected {
            username: username.to_string(),
        };
        self.alert_to_others(&alert);

        // Avisar a todos los cuartos donde estaba


        // Desconectar el cliente
        // Eliminarlo del broadcaster

    }
}

#[cfg(test)]
mod tests {}
