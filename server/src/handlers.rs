use crate::model::broadcaster::Broadcaster;
use crate::model::room::validate_room_name;
use crate::model::server_state::ServerState;
use crate::model::user::{User, validate_username};
use protocol::messages::client_message::ClientMessage;
use protocol::messages::responses::{Operation, OperationResult};
use protocol::messages::server_message::ServerMessage;
use protocol::status::user::UserStatus;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// Maneja la comunicación entre el servidor y un cliente.
///
/// - `username`: Nombre con el que el cliente se identifica en el servidor.
/// - `id`: Identificador del cliente en el servidor.
/// - `state`: Estado del servidor.
/// - `broadcaster`: Canal de comunicación del servidor con todos los clientes.
pub struct ClientHandler {
    username: Option<String>,
    id: usize,
    state: Arc<Mutex<ServerState>>,
    broadcaster: Arc<Mutex<Broadcaster>>,
}

impl ClientHandler {
    /// Crea un nuevo manejador de mensajes para el cliente
    pub fn new(
        id: usize,
        state: Arc<Mutex<ServerState>>,
        broadcaster: Arc<Mutex<Broadcaster>>,
    ) -> Self {
        Self {
            // La comunicación comienza antes de la identificación del cliente
            username: None,
            id,
            state,
            broadcaster,
        }
    }

    /// Verifica que el cliente se haya identificado
    ///
    /// Si el cliente se identificó, regresa el nombre de usuario.
    /// Si no se ha identificado, responde con operación inválida por no estar identificado.
    fn verify_identify(&self) -> Option<String> {
        if let Some(username) = &self.username {
            Some(username.clone())
        } else {
            let reply = ClientMessage::Response {
                operation: Operation::Invalid,
                result: OperationResult::NotIdentified,
                extra: None,
            };
            self.reply_to_client(&reply);
            None
        }
    }

    /// Regresa el identificador del cliente en el servidor.
    pub fn get_id(&self) -> usize {
        self.id
    }

    ///
    /// Métodos que requieren sincronizar el estado del servidor o el broadcaster
    ///

    ///
    /// Comunicación
    ///

    /// Responde al cliente con un mensaje
    ///
    /// - `reply`: Mensaje a enviar al cliente
    fn reply_to_client(&self, reply: &ClientMessage) {
        match self.broadcaster.lock() {
            Ok(b) => {
                b.send_message_to(vec![self.id], reply);
            }
            Err(_) => {
                println!("Error al enviar mensaje al cliente con id: {}", self.id);
            }
        }
    }

    /// Envía un mensaje a todos los demás clientes
    ///
    /// - `alert`: Mensaje a enviar a los demás clientes
    fn alert_to_others(&self, alert: &ClientMessage) {
        match self.broadcaster.lock() {
            Ok(b) => {
                b.send_message_to_all_except(self.id, alert);
            }
            Err(_) => {
                println!("Error al enviar mensaje a otros clientes");
            }
        }
    }

    /// Envía un mensaje a todos los usuarios de un cuarto
    ///
    /// - `roomname`: Nombre del cuarto al que se envía el mensaje
    /// - `msg`: Mensaje a enviar.
    fn send_to_room(&self, roomname: &str, msg: &ClientMessage) {
        // Obtenemos los identificadores de los usuarios en el cuarto con nombre dado.
        let ids_in_room = self.get_ids_in_room(roomname);

        match self.broadcaster.lock() {
            Ok(b) => {
                b.send_message_to(ids_in_room, msg);
            }
            Err(_) => {}
        }
    }

    /// Envía un mensaje a todos los usuarios dados en el vector de nombres de usuario.
    ///
    /// - `usernames`: Vector de nombres de usuario.
    /// - `msg`: Mensaje a enviar.
    fn send_to_users(&self, usernames: Vec<&str>, msg: &ClientMessage) {
        let ids_to_send = self.get_ids_by_usernames(usernames);

        match self.broadcaster.lock() {
            Ok(b) => {
                b.send_message_to(ids_to_send, msg);
            }
            Err(_) => {}
        }
    }

    ///
    /// Interacción con el estado del servidor
    ///

    /// Regresa los identificadores de los usuarios en un cuarto
    ///
    /// - `roomname`: Nombre del cuarto
    fn get_ids_in_room(&self, roomname: &str) -> Vec<usize> {
        match self.state.lock() {
            Ok(state) => state.get_ids_in_room(roomname),
            Err(_) => vec![],
        }
    }

    /// Regresa los identificadores de los usuarios dados en el vector de nombres de usuario.
    ///
    /// - `usernames`: Vector de nombres de usuario.
    fn get_ids_by_usernames(&self, usernames: Vec<&str>) -> Vec<usize> {
        match self.state.lock() {
            Ok(state) => state.get_ids_by_usernames(usernames),
            Err(_) => vec![],
        }
    }

    /// Verifica que un usuario esté en un cuarto
    ///
    /// - `username`: El usuario a verificar si está en el cuarto
    /// - `roomname`: El nombre del cuarto
    fn verify_is_in_room(&self, username: &str, roomname: &str) -> bool {
        match self.state.lock() {
            Ok(state) => state.user_is_in_room(username, roomname),
            Err(_) => false,
        }
    }

    /// Verifica que un cuarto exista
    ///
    /// - `roomname`: El nombre del cuarto a verificar
    fn verify_room_exists(&self, roomname: &str) -> bool {
        match self.state.lock() {
            Ok(state) => state.room_exists(roomname),
            Err(_) => false,
        }
    }

    /// Verifica si un usuario dado existe en el servidor
    ///
    /// - `username`: El nombre del usuario a verificar
    fn verify_user_exist(&self, username: &str) -> bool {
        match self.state.lock() {
            Ok(state) => state.user_exists(username),
            Err(_) => false,
        }
    }

    /// Verifica que un usuario haya sido invitado a un cuarto
    ///
    /// - `username`: El nombre del usuario a verificar
    /// - `roomname`: El nombre del cuarto
    fn verify_user_invited(&self, username: &str, roomname: &str) -> bool {
        match self.state.lock() {
            Ok(state) => state.user_invited_to_room(username, roomname),
            Err(_) => false,
        }
    }

    /// Agrega a un nuevo usuario al servidor
    ///
    /// - `user`: El usuario a agregar
    fn add_user_to_server(&self, user: User) {
        match self.state.lock() {
            Ok(mut state) => {
                state.add_new_user(user);
            }
            Err(_) => {}
        }
    }

    /// Agrega a un usuario a un cuarto
    ///
    /// - `username`: El nombre del usuario a agregar
    /// - `roomname`: El nombre del cuarto al que se agregará el usuario
    fn add_user_to_room(&mut self, username: &str, roomname: &str) {
        match self.state.lock() {
            Ok(mut state) => {
                state.add_user_to_room(username, roomname);
            }
            Err(_) => {}
        }
    }

    /// Elimina a un usuario de un cuarto
    ///
    /// - `username`: El nombre del usuario a eliminar
    /// - `roomname`: El nombre del cuarto del que se eliminará el usuario
    fn delete_user_from_room(&self, username: &str, roomname: &str) {
        match self.state.lock() {
            Ok(mut state) => {
                state.delete_user_from_room(username, roomname);
            }
            Err(_) => {}
        }
    }

    /// Elimina a un usuario del servidor
    ///
    /// - `username`: El nombre del usuario a eliminar
    fn delete_user_from_server(&self, username: &str) {
        match self.state.lock() {
            Ok(mut state) => {
                state.delete_user(username);
            }
            Err(_) => {}
        }
    }

    /// Actualiza el status de un usuario en el servidor
    ///
    /// - `username`: El nombre del usuario a actualizar
    /// - `new_status`: El nuevo status del usuario
    fn update_user_status(&self, username: &str, new_status: UserStatus) {
        match self.state.lock() {
            Ok(mut status) => {
                status.update_user_status(username, new_status);
            }
            Err(_) => {}
        }
    }

    /// Regresa la lista de cuartos donde está un usuario dado
    ///
    /// - `username`: El nombre del usuario
    fn get_rooms_user_is_in(&self, username: &str) -> Vec<String> {
        match self.state.lock() {
            Ok(state) => state.get_rooms_for_user(username),
            Err(_) => Vec::new(),
        }
    }

    /// Regresa un diccionario con los estados de los usuarios en un cuarto
    ///
    /// - `roomname`: El nombre del cuarto
    fn get_user_list_room(&self, roomname: &str) -> HashMap<String, UserStatus> {
        match self.state.lock() {
            Ok(state) => state.get_users_status_in_room(roomname),
            Err(_) => HashMap::new(),
        }
    }

    /// Regresa un diccionario con los estados de todos los usuarios
    fn get_user_list(&self) -> HashMap<String, UserStatus> {
        match self.state.lock() {
            Ok(state) => state.get_users_status(),
            Err(_) => HashMap::new(),
        }
    }

    /// Crea un nuevo cuarto en el servidor
    ///
    /// - `username`: El nombre del usuario que crea el cuarto
    /// - `roomname`: El nombre del cuarto a crear
    fn add_new_room(&self, username: &str, roomname: &str) {
        match self.state.lock() {
            Ok(mut state) => state.add_new_room(roomname, username),
            Err(_) => {}
        }
    }

    /// Elimina al cliente del broadcaster
    fn remove_client(&self) {
        match self.broadcaster.lock() {
            Ok(mut b) => {
                b.remove_client(self.id);
            }
            Err(_) => {}
        }
    }

    /// Agrega un usuario a la lista de invitados de un cuarto
    ///
    /// - `username`: El nombre del usuario a invitar
    /// - `roomname`: El nombre del cuarto al que se invita
    fn invite_to_room(&self, username: &str, roomname: &str) {
        match self.state.lock() {
            Ok(mut state) => {
                state.invite_user_to_room(username, roomname);
            }
            Err(_) => {}
        }
    }

    ///
    /// Manejo de mensajes entrantes desde el cliente
    ///

    /// Maneja los mensajes que recibe el servidor.
    ///
    /// - `msg`: Mensaje recibido del cliente
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
    /// Implementación de API del protocolo
    ///

    /// Maneja la identificación de un usuario.
    ///
    /// El cliente pide identificarse con el nombre de usuario dado.
    ///
    /// - `username`: Nombre de usuario del cliente
    fn handle_identify(&mut self, username: &str) {
        let reply: ClientMessage;

        if !validate_username(username) {
            reply = ClientMessage::Response {
                operation: Operation::Identify,
                result: OperationResult::Invalid,
                extra: Some("Nombre de usuario demasiado largo".to_string()),
            };
            self.reply_to_client(&reply);
            return;
        }

        // Verificamos si ya hay algún usuario con ese nombre
        if self.verify_user_exist(&username) {
            reply = ClientMessage::Response {
                operation: Operation::Identify,
                result: OperationResult::UserAlreadyExists,
                extra: Some(username.to_string()),
            };
            self.reply_to_client(&reply);
            return;
        }

        // El usuario por registrar aún no existe: lo creamos
        let new_user = User::new(username.to_string(), UserStatus::Active, self.id);

        // Agregar al usuario al servidor
        self.add_user_to_server(new_user);

        // Identificamos a este handler
        self.username = Some(username.to_string());

        reply = ClientMessage::Response {
            operation: Operation::Identify,
            result: OperationResult::Success,
            extra: Some(username.to_string()),
        };
        self.reply_to_client(&reply);

        // Avisamos a todos los demás
        let alert = ClientMessage::NewUser {
            username: username.to_string(),
        };
        self.alert_to_others(&alert);
    }

    /// Maneja el cambio de estado de un usuario
    ///
    /// El cliente pide cambiar su estado.
    ///
    /// - `new_status`: El nuevo estado del usuario
    fn handle_status(&mut self, new_status: UserStatus) {
        let username = match self.verify_identify() {
            Some(username) => username,
            None => return,
        };

        // Cambiamos el status
        self.update_user_status(&username, new_status.clone());

        // Alertamos a los demás
        let alert = ClientMessage::NewStatus {
            username,
            status: new_status,
        };
        self.alert_to_others(&alert);
    }

    /// Maneja users
    ///
    /// El cliente pide la lista de usuarios conectados con sus estados
    fn handle_users(&mut self) {
        let _ = match self.verify_identify() {
            Some(username) => username,
            None => return,
        };

        let users_status = self.get_user_list();
        let reply = ClientMessage::UserList {
            users: users_status,
        };
        self.reply_to_client(&reply);
    }

    /// Maneja text
    ///
    /// El cliente pide enviar un mensaje privado a otro usuario.
    ///
    /// - `username_to`: El nombre del usuario al que se le va a enviar el mensaje.
    /// - `text`: El texto del mensaje
    fn handle_text(&mut self, username_to: &str, text: &str) {
        let username_from = match self.verify_identify() {
            Some(username) => username,
            None => return,
        };

        let reply: ClientMessage;

        // Verificar que el destinatario exista en el servidor
        if !self.verify_user_exist(&username_to) {
            reply = ClientMessage::Response {
                operation: Operation::Text,
                result: OperationResult::NoSuchUser,
                extra: Some(username_to.to_string()),
            };
            self.reply_to_client(&reply);
            return;
        }

        // El destinatario existe
        reply = ClientMessage::TextFrom {
            username: username_from.to_string(),
            text: text.to_string(),
        };
        self.send_to_users(vec![username_to], &reply);
    }

    /// Maneja public text
    ///
    /// El cliente pide enviar un mensaje público a todos los usuarios conectados.
    ///
    /// - `text`: El texto del mensaje
    fn handle_public_text(&mut self, text: &str) {
        let username_from = match self.verify_identify() {
            Some(username) => username,
            None => return,
        };

        let reply = ClientMessage::PublicTextFrom {
            username: username_from,
            text: text.to_string(),
        };
        self.alert_to_others(&reply);
    }

    /// Maneja new room
    ///
    /// El cliente pide crear un nuevo cuarto.
    ///
    /// - `roomname`: El nombre del nuevo cuarto
    fn handle_new_room(&mut self, roomname: &str) {
        let username = match self.verify_identify() {
            Some(username) => username,
            None => return,
        };

        let reply: ClientMessage;

        if !validate_room_name(roomname) {
            reply = ClientMessage::Response {
                operation: Operation::NewRoom,
                result: OperationResult::Invalid,
                extra: Some("Nombre de cuarto demasiado largo".to_string()),
            };
            self.reply_to_client(&reply);
            return;
        }

        // Verificar si el cuarto ya existe
        if self.verify_room_exists(&roomname) {
            reply = ClientMessage::Response {
                operation: Operation::NewRoom,
                result: OperationResult::RoomAlreadyExists,
                extra: Some(roomname.to_string()),
            };
            self.reply_to_client(&reply);
            return;
        }

        // El cuarto no existe: lo creamos
        self.add_new_room(&username, &roomname);

        reply = ClientMessage::Response {
            operation: Operation::NewRoom,
            result: OperationResult::Success,
            extra: Some(roomname.to_string()),
        };
        self.reply_to_client(&reply);
    }

    /// Maneja invite
    ///
    /// El cliente invita a otros usuarios a un cuarto.
    ///
    /// - `roomname`: Nombre del cuarto al que se invita
    /// - `usernames`: Vector de usuarios a los que se invita
    fn handle_invite(&mut self, roomname: &str, usernames: Vec<String>) {
        let username = match self.verify_identify() {
            Some(username) => username,
            None => return,
        };

        let mut reply: ClientMessage;

        // Si el cuarto no existe
        if !self.verify_room_exists(&roomname) {
            reply = ClientMessage::Response {
                operation: Operation::Invite,
                result: OperationResult::NoSuchRoom,
                extra: Some(roomname.to_string()),
            };
            self.reply_to_client(&reply);
            return;
        }

        // El cuarto existe

        // Si no está en el cuarto no puede invitar
        if !self.verify_is_in_room(&username, &roomname) {
            reply = ClientMessage::Response {
                operation: Operation::Invite,
                result: OperationResult::NotJoined,
                extra: Some(roomname.to_string()),
            };
            self.reply_to_client(&reply);
            return;
        }

        // El que invita sí está en el cuarto

        // Usuarios que no están en el cuarto
        let mut users_to_invite: Vec<&str> = Vec::new();
        // Usuarios que no exiten
        let mut users_not_exist: Vec<&str> = Vec::new();

        for user_invited in &usernames {
            if self.verify_is_in_room(user_invited, roomname) {
                continue;
            }
            if !self.verify_user_exist(user_invited) {
                users_not_exist.push(user_invited.as_str())
            } else {
                users_to_invite.push(user_invited.as_str())
            }
        }

        // Notificar de usuarios que no existen
        for user_not_exist in users_not_exist {
            reply = ClientMessage::Response {
                operation: Operation::Invite,
                result: OperationResult::NoSuchUser,
                extra: Some(user_not_exist.to_string()),
            };
            self.reply_to_client(&reply);
        }

        // Agregarlos a la lista de invitados del cuarto
        for user_to_invite in &users_to_invite {
            self.invite_to_room(user_to_invite, &roomname);
        }

        // Enviar invitación a usuarios existentes que no están en el cuarto
        let reply = ClientMessage::Invitation {
            username: username.to_string(),
            roomname: roomname.to_string(),
        };
        self.send_to_users(users_to_invite, &reply);
    }

    /// Maneja join_room
    ///
    /// El cliente pide (acepta) unirse a un cuarto.
    ///
    /// - `roomname`: Nombre del cuarto al que se quiere unir
    fn handle_join_room(&mut self, roomname: String) {
        let username = match self.verify_identify() {
            Some(username) => username,
            None => return,
        };

        let reply: ClientMessage;

        // Verificar que el cuarto existe
        if !self.verify_room_exists(&roomname) {
            reply = ClientMessage::Response {
                operation: Operation::JoinRoom,
                result: OperationResult::NoSuchRoom,
                extra: Some(roomname.to_string()),
            };
            self.reply_to_client(&reply);
            return;
        }

        // El cuarto existe

        // Verificar que este usuario haya sido invitado al cuarto roomname
        if !self.verify_user_invited(&username, &roomname) {
            reply = ClientMessage::Response {
                operation: Operation::JoinRoom,
                result: OperationResult::NotInvited,
                extra: Some(roomname.to_string()),
            };
            self.reply_to_client(&reply);
            return;
        }

        // El usuario fue invitado

        // Agregar al usuario al cuarto
        self.add_user_to_room(&username, &roomname);

        reply = ClientMessage::Response {
            operation: Operation::JoinRoom,
            result: OperationResult::Success,
            extra: Some(roomname.to_string()),
        };
        self.reply_to_client(&reply);

        // Avisar en el cuarto que se unió
        let alert = ClientMessage::JoinedRoom {
            username: username.to_string(),
            roomname: roomname.to_string(),
        };
        self.send_to_room(&roomname, &alert)
    }

    /// Maneja room_users
    ///
    /// El cliente pide la lista de usuarios en un cuarto
    ///
    /// - `roomname`: Nombre del cuarto
    fn handle_room_users(&mut self, roomname: String) {
        let username = match self.verify_identify() {
            Some(username) => username,
            None => return,
        };

        let reply: ClientMessage;

        // Verificar que el cuarto existe
        if !self.verify_room_exists(&roomname) {
            reply = ClientMessage::Response {
                operation: Operation::RoomUsers,
                result: OperationResult::NoSuchRoom,
                extra: Some(roomname.to_string()),
            };
            self.reply_to_client(&reply);
            return;
        }

        // El cuarto existe

        // Verificar que el usuario este en el cuarto
        if !self.verify_is_in_room(&username, &roomname) {
            reply = ClientMessage::Response {
                operation: Operation::RoomUsers,
                result: OperationResult::NotJoined,
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

    /// Maneja room text
    ///
    /// El cliente pide enviar un mensaje a un cuarto.
    ///
    /// - `roomname`: Nombre del cuarto
    /// - `text`: Texto del mensaje
    fn handle_room_text(&mut self, roomname: String, text: String) {
        let username = match self.verify_identify() {
            Some(username) => username,
            None => return,
        };

        // Verificar que el cuarto existe
        if !self.verify_room_exists(&roomname) {
            let reply = ClientMessage::Response {
                operation: Operation::RoomText,
                result: OperationResult::NoSuchRoom,
                extra: Some(roomname.to_string()),
            };
            self.reply_to_client(&reply);
            return;
        }

        // El cuarto existe

        // Verificar que el usuario esté en el cuarto
        if !self.verify_is_in_room(&username, &roomname) {
            let reply = ClientMessage::Response {
                operation: Operation::RoomText,
                result: OperationResult::NotJoined,
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

    /// Maneja leave room
    ///
    /// El cliente pide salir de un cuarto.
    ///
    /// - `roomname`: Nombre del cuarto
    fn handle_leave_room(&mut self, roomname: String) {
        let username = match self.verify_identify() {
            Some(username) => username,
            None => return,
        };

        let reply: ClientMessage;

        // Verificar que el cuarto existe
        if !self.verify_room_exists(&roomname) {
            reply = ClientMessage::Response {
                operation: Operation::LeaveRoom,
                result: OperationResult::NoSuchRoom,
                extra: Some(roomname.to_string()),
            };
            self.reply_to_client(&reply);
            return;
        }

        // El cuarto existe

        // Verificar que el usuario esté en el cuarto
        if !self.verify_is_in_room(&username, &roomname) {
            reply = ClientMessage::Response {
                operation: Operation::LeaveRoom,
                result: OperationResult::NotJoined,
                extra: Some(roomname.to_string()),
            };
            self.reply_to_client(&reply);
            return;
        }

        // El usuario está en el cuarto

        // Avisar a los demás
        let alert = ClientMessage::LeftRoom {
            roomname: roomname.to_string(),
            username: username.to_string(),
        };
        self.send_to_room(&roomname, &alert);

        // Eliminarlo de la lista de usuarios del cuarto
        // Lo dejamos en invitados por si quiere volver,
        // pues el protocolo no especifica lo contrario
        self.delete_user_from_room(&username, &roomname);
    }

    /// Maneja disconnect
    ///
    /// El cliente pide desconectarse del servidor.
    pub fn handle_disconnect(&mut self) {
        let username = match self.verify_identify() {
            Some(username) => username,
            None => return,
        };

        let mut alert: ClientMessage;

        // Eliminarlo de todos los cuartos donde esté y avisar
        let user_rooms = self.get_rooms_user_is_in(&username);
        for room_name in user_rooms {
            self.delete_user_from_room(&username, &room_name);

            alert = ClientMessage::LeftRoom {
                roomname: room_name.to_string(),
                username: username.to_string(),
            };
            self.send_to_room(room_name.as_str(), &alert);
        }

        // Eliminarlo de la lista de usuarios del servidor
        self.delete_user_from_server(&username);

        // Avisar que se desconectó
        alert = ClientMessage::Disconnected {
            username: username.to_string(),
        };
        self.alert_to_others(&alert);

        // Eliminarlo del broadcaster
        self.remove_client();
    }
}

#[cfg(test)]
mod tests {}
