use crate::model::room::Room;
use crate::model::user::User;
use protocol::status::user::UserStatus;
use std::collections::HashMap;
use std::sync::atomic::{AtomicUsize, Ordering};

///   Estado del servidor.
///
/// - `users`: Los usuarios activos en el servidor.
/// - `rooms`: Los cuartos creados en el servidor.
/// - `conn_counter`: Contador de conexiones.
pub struct ServerState {
    users: HashMap<String, User>,
    rooms: HashMap<String, Room>,
    conn_counter: AtomicUsize,
}

impl ServerState {
    /// Crea un nuevo estado del servidor
    ///
    /// Inicialmente el servidor no tiene usuarios, cuartos y tiene 0 conexiones.
    pub fn new() -> Self {
        Self {
            users: HashMap::new(),
            rooms: HashMap::new(),
            conn_counter: AtomicUsize::new(0),
        }
    }

    /// Regresa el siguiente id disponible.
    pub fn get_next_id(&self) -> usize {
        self.conn_counter.fetch_add(1, Ordering::SeqCst)
    }

    /// Regresa una referencia a los usuarios activos en el servidor.
    ///
    pub fn get_users(&self) -> &HashMap<String, User> {
        &self.users
    }

    /// Agrega un nuevo usuario a los usuarios activos en el servidor.
    ///
    /// - `user`: El usuario a agregar.
    pub fn insert_user(&mut self, user: User) {
        self.users.insert(user.get_username(), user);
    }

    /// Cambia el status de un usuario
    ///
    /// - `username`: El usuario que actualiza su status
    /// - `new_status`: El nuevo status del usuario
    pub fn change_user_status(&mut self, username: &str, new_status: UserStatus) {
        if let Some(user) = self.users.get_mut(username) {
            user.set_status(new_status);
        }
    }
    
    /// Regresa un diccionario con el status de todos los usuarios
    pub fn get_users_status(&self) -> HashMap<String, UserStatus> {
        let status_map = self
            .users
            .iter()
            .map(|(username, user)| (username.clone(), user.get_status()))
            .collect::<HashMap<String, UserStatus>>();
        status_map
    }
    
    /// Regresa una referencia mutable a los cuartos creados en el servidor.
    pub fn get_rooms(&mut self) -> &mut HashMap<String, Room> {
        &mut self.rooms
    }
    
    /// Elimina un usuario del servidor
    /// 
    /// - `username`: El usuario a eliminar.
    /// 
    /// Regresa true si el usuario fue eliminado exitosamente, false si no se encontró el usuario.
    pub fn remove_user(&mut self, username: &str) -> bool {
        self.users.remove(username).is_some()
    }
    
    /// Agrega un nuevo cuarto al servidor.
    /// 
    /// - `room`: El cuarto a agregar.
    pub fn add_new_room(&mut self, room: Room) {
        self.rooms.insert(room.get_room_name(), room);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use protocol::status::user::UserStatus;

    #[test]
    fn test_server_init_state() {
        let state = ServerState::new();
        assert_eq!(state.get_users().len(), 0);
        assert_eq!(state.get_next_id(), 0);
    }

    #[test]
    fn test_server_insert_user() {
        let mut state = ServerState::new();
        let user = User::new("user_1".to_string(), UserStatus::Active, 0);
        state.insert_user(user);
        assert_eq!(state.get_users().len(), 1);
    }

    #[test]
    fn test_server_counter_after_insert() {
        let mut state = ServerState::new();
        let user = User::new("user_1".to_string(), UserStatus::Active, 0);
        state.insert_user(user);
        assert_eq!(state.get_next_id(), 0);
    }
}
