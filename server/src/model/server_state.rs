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

    /// Regresa un vector de ids de los nombres de usuario dados
    ///
    /// - `usernames`: Vector de nombres de usuario.
    pub fn get_ids_by_usernames(&self, usernames: Vec<&str>) -> Vec<usize> {
        usernames
            .iter()
            .filter_map(|username| self.users.get(*username).map(|u| u.get_id()))
            .collect::<Vec<_>>()
    }

    /// Agrega un nuevo usuario a los usuarios activos en el servidor.
    ///
    /// - `user`: El usuario a agregar.
    pub fn add_new_user(&mut self, user: User) {
        self.users.insert(user.get_username(), user);
    }

    /// Cambia el status de un usuario
    ///
    /// - `username`: El usuario que actualiza su status
    /// - `new_status`: El nuevo status del usuario
    pub fn update_user_status(&mut self, username: &str, new_status: UserStatus) {
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

    /// Regresa un diccionario con el status de todos los usuarios en un cuarto
    ///
    /// - `roomname`: El nombre del cuarto
    pub fn get_users_status_in_room(&self, roomname: &str) -> HashMap<String, UserStatus> {
        if let Some(room) = self.rooms.get(roomname) {
            let users_in_room = room.get_users();
            let users_status = self.get_users_status();
            users_in_room
                .iter()
                .filter_map(|(_, username)| {
                    users_status
                        .get(username)
                        .map(|status| (username.clone(), status.clone()))
                })
                .collect()
        } else {
            HashMap::new()
        }
    }

    /// Regresa los ids de los usuarios en el cuarto de nombre dado.
    ///
    /// - `roomname`: El nombre del cuarto.
    pub fn get_ids_in_room(&self, roomname: &str) -> Vec<usize> {
        if let Some(room) = self.rooms.get(roomname) {
            return room.get_users_ids();
        }
        Vec::new()
    }

    /// Elimina un usuario del servidor
    ///
    /// - `username`: El usuario a eliminar.
    ///
    /// Regresa true si el usuario fue eliminado exitosamente, false si no se encontró el usuario.
    pub fn delete_user(&mut self, username: &str) -> bool {
        self.users.remove(username).is_some()
    }

    /// Agrega un nuevo cuarto al servidor.
    ///
    /// - `roomname`: El nombre del cuarto a crear
    /// - `username`: El usuario que crea el cuarto
    pub fn add_new_room(&mut self, roomname: &str, username: &str) {
        if let Some(user) = self.users.get(username) {
            let mut room = Room::new();

            room.add_user(username, user.get_id());
            self.rooms.insert(roomname.to_string(), room);
        }
    }

    /// Verifica si un usuario está en el cuarto dado.
    ///
    /// - `username`: El usuario a verificar
    /// - `roomname`: El nombre del cuarto
    pub fn user_is_in_room(&self, username: &str, roomname: &str) -> bool {
        if let Some(room) = self.rooms.get(roomname) {
            room.is_in(username)
        } else {
            false
        }
    }

    /// Verifica si existe un cuarto con el nombre dado
    ///
    /// - `roomname`: El nombre del cuarto
    pub fn room_exists(&self, roomname: &str) -> bool {
        self.rooms.contains_key(roomname)
    }

    /// Verifica que un nombre de usuario esté registrado
    ///
    /// - `username`: El nombre de usuario a verificar
    pub fn user_exists(&self, username: &str) -> bool {
        self.users.contains_key(username)
    }

    /// Verifica si un usuario está en la lista de invitados del cuarto dado.
    ///
    /// - `username`: El nombre del usuario
    /// - `roomname`: El nombre del cuarto
    pub fn user_invited_to_room(&self, username: &str, roomname: &str) -> bool {
        if let Some(room) = self.rooms.get(roomname) {
            room.is_invited(username)
        } else {
            false
        }
    }

    /// Agrega a un usuario con nombre dado a un cuarto.
    ///
    /// - `username`: El nombre del usuario
    /// - `roomname`: El nombre del cuarto
    pub fn add_user_to_room(&mut self, username: &str, roomname: &str) {
        if let Some(room) = self.rooms.get_mut(roomname) {
            if let Some(user) = self.users.get(username) {
                room.add_user(username, user.get_id());
            }
        }
    }

    /// Elimina a un usuario de un cuarto
    ///
    /// - `username`: El nombre del usuario a eliminar
    /// - `roomname`: El nombre del cuarto del que se eliminará el usuario
    pub fn delete_user_from_room(&mut self, username: &str, roomname: &str) {
        if let Some(user) = self.users.get(username) {
            if let Some(room) = self.rooms.get_mut(roomname) {
                room.remove_user(user.get_id());
            }
        }
    }

    /// Regresa los nombres de los cuartos donde está un usuario dado.
    ///
    /// - `username`: El nombre del usuario
    pub fn get_rooms_for_user(&self, username: &str) -> Vec<String> {
        let mut rooms = Vec::new();
        for (roomname, room) in self.rooms.iter() {
            if room.is_in(username) {
                rooms.push(roomname.clone());
            }
        }
        rooms
    }

    /// Invita a un usuario a un cuarto
    ///
    /// - `username`: El nombre de usuario a invitar
    /// - `roomname`: El nombre del cuarto al que se invita
    pub fn invite_user_to_room(&mut self, username: &str, roomname: &str) {
        if let Some(_) = self.users.get(username) {
            if let Some(room) = self.rooms.get_mut(roomname) {
                room.invite_user(username)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use protocol::status::user::UserStatus;

    #[test]
    fn test_server_init_state() {
        let state = ServerState::new();
        assert_eq!(state.get_next_id(), 0);
    }

    #[test]
    fn test_server_insert_user() {
        let mut state = ServerState::new();
        let user = User::new("user_1".to_string(), UserStatus::Active, 0);
        state.add_new_user(user);
    }

    #[test]
    fn test_server_counter_after_insert() {
        let mut state = ServerState::new();
        let user = User::new("user_1".to_string(), UserStatus::Active, 0);
        state.add_new_user(user);
        assert_eq!(state.get_next_id(), 0);
    }
}
