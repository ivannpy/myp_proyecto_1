use crate::model::connection_state::ConnectionState;
use crate::model::message::ChatMessage;
use crate::model::room::RemoteRoom;
use crate::model::users::{LocalUser, RemoteUser};
use protocol::status::user::UserStatus;
use std::collections::HashMap;

/// Modelo del chat
///
/// - `local_user`: El usuario local
/// - `remote_users`: Los usuarios remotos
/// - `rooms`: Las habitaciones disponibles
/// - `messages`: Los mensajes enviados y recibidos.
/// - `connection_state`: Estado de conexión de la aplicación
#[derive(Debug, Clone)]
pub struct ChatModel {
    local_user: LocalUser,
    remote_users: HashMap<String, RemoteUser>,
    rooms: HashMap<String, RemoteRoom>,
    messages: Vec<ChatMessage>,
    connection_state: ConnectionState,
}

impl ChatModel {
    /// Crea un nuevo chat
    pub fn new() -> Self {
        Self {
            local_user: LocalUser::new(String::from(""), UserStatus::Active),
            remote_users: HashMap::new(),
            rooms: HashMap::new(),
            messages: Vec::new(),
            connection_state: ConnectionState::Disconnected,
        }
    }

    /// Agrega a un nuevo usuario remoto al chat
    pub fn add_remote_user(&mut self, username: String, status: UserStatus) {
        self.remote_users
            .insert(username.clone(), RemoteUser::new(username, status));
    }

    /// Elimina a un usuario del chat
    pub fn remove_remote_user(&mut self, username: &str) {
        self.remote_users.remove(username);
    }

    /// Actualiza el estado de un usuario en el chat
    pub fn update_user_status(&mut self, username: &str, status: UserStatus) {
        if let Some(user) = self.remote_users.get_mut(username) {
            user.set_status(status);
        }
    }

    /// Actualiza el estado del usuario local
    pub fn update_local_user_status(&mut self, status: UserStatus) {
        self.local_user.set_status(status);
    }

    /// Actualiza el estado de conexión del usuario local
    pub fn update_local_user_connected(&mut self, connected: bool) {
        self.local_user.set_connected(connected);
    }

    /// Agrega un nuevo mensaje
    pub fn add_message(&mut self, message: ChatMessage) {
        self.messages.push(message);
    }

    /// Agrega un nuevo cuarto
    pub fn add_room(&mut self, roomname: String, is_joined: bool, is_invited: bool) {
        self.rooms.insert(
            roomname.clone(),
            RemoteRoom::new(roomname, is_invited, is_joined),
        );
    }

    /// Agrega al usuario local a un cuarto
    pub fn join_room(&mut self, roomname: &str) {
        if let Some(room) = self.rooms.get_mut(roomname) {
            room.set_is_invited(true);
            room.set_is_joined(true);
        }
    }

    /// Saca al usuario local de un cuarto y elimina a todos los usuarios de ese cuarto.
    pub fn leave_room(&mut self, roomname: &str) {
        if let Some(room) = self.rooms.get_mut(roomname) {
            room.set_is_joined(false);
            room.set_users(HashMap::new());
        }
    }

    /// Agrega a un usuario remoto a un cuarto
    pub fn add_room_member(&mut self, roomname: &str, username: String, status: UserStatus) {
        if let Some(room) = self.rooms.get_mut(roomname) {
            room.add_new_user(username, status);
        }
    }

    /// Elimina a un usuario remoto de un cuarto
    pub fn remove_room_member(&mut self, roomname: &str, username: &str) {
        if let Some(room) = self.rooms.get_mut(roomname) {
            room.remove_user(username);
        }
    }

    /// Regresa el estado de conexión del usuario local
    pub fn get_connection_state(&self) -> ConnectionState {
        self.connection_state.clone()
    }

    /// Fija el estado de conexión de un usuario
    pub fn set_connection_state(&mut self, state: ConnectionState) {
        self.connection_state = state;
    }

    /// Regresa una referencia al usuario local
    pub fn get_local_user(&self) -> &LocalUser {
        &self.local_user
    }

    /// Regresa una referencia al diccionario de usuarios remotos
    pub fn get_remote_users(&self) -> &HashMap<String, RemoteUser> {
        &self.remote_users
    }

    /// Regresa una referencia al diccionario de cuarto
    pub fn get_rooms(&self) -> &HashMap<String, RemoteRoom> {
        &self.rooms
    }

    /// Regresa una referencia mutable a un cuarto, si este existe
    pub fn get_rooms_mut(&mut self) -> Option<&mut HashMap<String, RemoteRoom>> {
        Some(&mut self.rooms)
    }

    /// Elimina a los usuarios remotos
    pub fn clean_remote_users(&mut self) {
        self.remote_users.clear();
    }

    /// Fija el nombre de usuario del usuario local
    pub fn set_local_username(&mut self, username: &str) {
        self.local_user.set_username(username.to_string());
    }
}
