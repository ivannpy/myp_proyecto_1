use crate::client_error::ClientError;
use crate::model::chat_model::ChatModel;
use crate::model::connection_state::ConnectionState;
use crate::model::message::ChatMessage;
use crate::network_connection::{NetworkReader, NetworkWriter, connect};
use crate::view::chat_view::ChatView;
use protocol::messages::client_message::ClientMessage;
use protocol::messages::responses::{Operation, OperationResult};
use protocol::messages::server_message::ServerMessage;
use protocol::status::user::UserStatus;
use std::collections::HashMap;

pub struct ChatController<V: ChatView> {
    pub model: ChatModel,
    pub view: V,
    pub writer: Option<NetworkWriter>,
}

impl<V: ChatView> ChatController<V> {
    pub fn new(view: V) -> Self {
        Self {
            model: ChatModel::new(),
            view,
            writer: None,
        }
    }

    /// Conectar al servidor. Devuelve el NetworkReader para que se maneje en otro hilo.
    pub fn connect(&mut self, addr: &str, port: u16) -> Result<NetworkReader, ClientError> {
        let (reader, writer) = connect(addr, port)?;
        self.writer = Some(writer);
        self.model.update_local_user_connected(true);
        self.model.set_connection_state(ConnectionState::Connected);
        self.view
            .update_connection_state(&self.model.get_connection_state());
        Ok(reader)
    }

    /// Identificarse en el servidor.
    /// Necesita el reader temporalmente para leer la respuesta sincrónica.
    pub fn identify(
        &mut self,
        username: String,
        reader: &mut NetworkReader,
    ) -> Result<(), ClientError> {
        let msg_to_send = ServerMessage::Identify { username };

        if let Some(writer) = &mut self.writer {
            writer.send_message(&msg_to_send)?;
        } else {
            return Err(ClientError::ConnectionError);
        }

        let response = reader.receive_message()?;
        self.handle_server_message(response);
        Ok(())
    }

    // Cambiar estado
    pub fn set_status(&mut self, status: UserStatus) {
        if let Some(writer) = &mut self.writer {
            let _ = writer.send_message(&ServerMessage::Status {
                status: status.clone(),
            });
            self.model.update_local_user_status(status);
        }
    }

    // Solicitar lista de usuarios
    pub fn request_users(&mut self) {
        if let Some(writer) = &mut self.writer {
            let _ = writer.send_message(&ServerMessage::Users);
        }
    }

    // Enviar mensaje privado
    pub fn send_private_message(&mut self, username: String, text: String) {
        if let Some(writer) = &mut self.writer {
            let _ = writer.send_message(&ServerMessage::Text { username, text });
        }
    }

    // Enviar mensaje público
    pub fn send_public_message(&mut self, text: String) {
        if let Some(writer) = &mut self.writer {
            let _ = writer.send_message(&ServerMessage::PublicText { text });
        }
    }

    // Crear cuarto
    pub fn create_room(&mut self, roomname: String) {
        if let Some(writer) = &mut self.writer {
            let _ = writer.send_message(&ServerMessage::NewRoom { roomname });
        }
    }

    // Invitar usuarios a cuarto
    pub fn invite_users(&mut self, roomname: String, usernames: Vec<String>) {
        if let Some(writer) = &mut self.writer {
            let _ = writer.send_message(&ServerMessage::Invite {
                roomname,
                usernames,
            });
        }
    }

    // Unirse a cuarto
    pub fn join_room(&mut self, roomname: String) {
        if let Some(writer) = &mut self.writer {
            let _ = writer.send_message(&ServerMessage::JoinRoom { roomname });
        }
    }

    // Solicitar usuarios de cuarto
    pub fn request_room_users(&mut self, roomname: String) {
        if let Some(writer) = &mut self.writer {
            let _ = writer.send_message(&ServerMessage::RoomUsers { roomname });
        }
    }

    // Enviar mensaje a cuarto
    pub fn send_room_message(&mut self, roomname: String, text: String) {
        if let Some(writer) = &mut self.writer {
            let _ = writer.send_message(&ServerMessage::RoomText { roomname, text });
        }
    }

    // Abandonar cuarto
    pub fn leave_room(&mut self, roomname: String) {
        if let Some(writer) = &mut self.writer {
            let _ = writer.send_message(&ServerMessage::LeaveRoom { roomname });
        }
    }

    // Desconectar
    pub fn disconnect(&mut self) {
        if let Some(writer) = &mut self.writer {
            let _ = writer.send_message(&ServerMessage::Disconnect);
        }

        self.writer = None;
        self.model
            .set_connection_state(ConnectionState::Disconnected);
        self.model.update_local_user_connected(false);
        self.view
            .update_connection_state(&self.model.get_connection_state());
    }

    // Procesar mensaje recibido del servidor
    pub fn handle_server_message(&mut self, msg: ClientMessage) {
        match msg {
            ClientMessage::Response {
                operation,
                result,
                extra,
            } => {
                self.handle_response(operation, result, extra);
            }
            ClientMessage::NewUser { username } => {
                self.handle_new_user(username);
            }
            ClientMessage::NewStatus { username, status } => {
                self.handle_new_status(username, status);
            }
            ClientMessage::UserList { users } => {
                self.handle_user_list(users);
            }
            ClientMessage::TextFrom { username, text } => {
                self.handle_text_from(username, text);
            }
            ClientMessage::PublicTextFrom { username, text } => {
                self.handle_public_text_from(username, text);
            }
            ClientMessage::JoinedRoom { roomname, username } => {
                self.handle_joined_room(roomname, username);
            }
            ClientMessage::RoomUserList { roomname, users } => {
                self.handle_room_user_list(roomname, users);
            }
            ClientMessage::RoomTextFrom {
                roomname,
                username,
                text,
            } => {
                self.handle_room_text_from(roomname, username, text);
            }
            ClientMessage::LeftRoom { roomname, username } => {
                self.handle_left_room(roomname, username);
            }
            ClientMessage::Disconnected { username } => {
                self.handle_disconnected(username);
            }
            ClientMessage::Invitation { roomname, username } => {
                self.handle_invitation(roomname, username);
            }
        }
    }

    // --- Handlers individuales (sin cambios) ---

    fn handle_response(
        &mut self,
        operation: Operation,
        result: OperationResult,
        extra: Option<String>,
    ) {
        match (&operation, &result) {
            (Operation::Identify, OperationResult::Success) => {
                if let Some(username) = extra {
                    self.model.set_connection_state(ConnectionState::Identified);
                    self.view
                        .update_connection_state(&self.model.get_connection_state());
                    self.view
                        .show_notification(&format!("Identificado como {}", username));
                    self.model.set_local_username(&username);
                }
            }
            (Operation::Identify, OperationResult::UserAlreadyExists) => {
                self.view.show_error("El nombre de usuario ya existe");
            }
            (Operation::NewRoom, OperationResult::Success) => {
                if let Some(roomname) = extra {
                    self.model.add_room(roomname.clone(), true, false);
                    self.view.add_room(&self.model.get_rooms()[&roomname]);
                    self.view
                        .show_notification(&format!("Cuarto '{}' creado", roomname));
                }
            }
            (Operation::JoinRoom, OperationResult::Success) => {
                if let Some(roomname) = extra {
                    self.model.join_room(&roomname);
                    self.view
                        .show_notification(&format!("Te uniste al cuarto '{}'", roomname));
                }
            }
            (_, OperationResult::Success) => {
                // Otras operaciones exitosas
                self.view
                    .show_notification(&format!("Operación {:?} exitosa", operation));
            }
            _ => {
                self.view
                    .show_error(&format!("Error en operación {:?}: {:?}", operation, result));
            }
        }
    }

    fn handle_new_user(&mut self, username: String) {
        self.model
            .add_remote_user(username.clone(), UserStatus::Active);
        self.view
            .add_user(&self.model.get_remote_users()[&username]);
    }

    fn handle_new_status(&mut self, username: String, status: UserStatus) {
        self.model.update_user_status(&username, status);
        if let Some(user) = self.model.get_remote_users().get(&username) {
            self.view.update_user(&username, user);
        }
    }

    fn handle_user_list(&mut self, users: HashMap<String, UserStatus>) {
        self.model.clean_remote_users();
        for (username, status) in users {
            self.model.add_remote_user(username.clone(), status);
        }
        self.view.update_user_list(&self.model.get_remote_users());
    }

    fn handle_text_from(&mut self, username: String, text: String) {
        let message = ChatMessage::Private {
            from: username.clone(),
            text,
        };
        self.model.add_message(message.clone());
        self.view.show_message(&message);
    }

    fn handle_public_text_from(&mut self, username: String, text: String) {
        let message = ChatMessage::Public {
            from: username,
            text,
        };
        self.model.add_message(message.clone());
        self.view.show_message(&message);
    }

    fn handle_joined_room(&mut self, roomname: String, username: String) {
        if username == self.model.get_local_user().get_username() {
            self.model.join_room(&roomname);
        } else {
            self.model
                .add_room_member(&roomname, username.clone(), UserStatus::Active);
        }

        if let Some(room) = self.model.get_rooms().get(&roomname) {
            self.view.update_room(&roomname, room);
        }
    }

    fn handle_room_user_list(&mut self, roomname: String, users: HashMap<String, UserStatus>) {
        if let Some(rooms) = self.model.get_rooms_mut() {
            if let Some(room) = rooms.get_mut(&roomname) {
                room.set_users(users);
                self.view.show_room_members(&roomname, &room.get_users());
            }
        }
    }

    fn handle_room_text_from(&mut self, roomname: String, username: String, text: String) {
        let message = ChatMessage::Room {
            roomname: roomname.clone(),
            from: username,
            text,
        };
        self.model.add_message(message.clone());
        self.view.show_message(&message);
    }

    fn handle_left_room(&mut self, roomname: String, username: String) {
        if username == self.model.get_local_user().get_username() {
            self.model.leave_room(&roomname);
        } else {
            self.model.remove_room_member(&roomname, &username);
        }

        if let Some(room) = self.model.get_rooms().get(&roomname) {
            self.view.update_room(&roomname, room);
        }
    }

    fn handle_disconnected(&mut self, username: String) {
        self.model.remove_remote_user(&username);
        self.view.remove_user(&username);

        if let Some(rooms) = self.model.get_rooms_mut() {
            for room in rooms.values_mut() {
                room.remove_user(&username);
            }
        }
    }

    fn handle_invitation(&mut self, roomname: String, username: String) {
        // Registrar el cuarto como invitado (no unido aún)
        self.model.add_room(roomname.clone(), false, true);
        self.view.show_notification(&format!(
            "{} te invitó al cuarto '{}'. Usa /join {} para unirte.",
            username, roomname, roomname
        ));
    }
}
