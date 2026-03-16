use std::collections::{HashMap, HashSet};

/// Longitud máxima del nombre de un cuarto
pub const MAX_ROOM_NAME_LEN: usize = 16;

/// Valida que un nombre del cuarto tenga la longitud correcta
pub fn validate_room_name(room: &str) -> bool {
    room.len() <= MAX_ROOM_NAME_LEN
}

/// Un cuarto en el chat.
///
/// - `users`: Los usuarios que están en el cuarto
/// - `room_name`: El nombre del cuarto
/// - `invited`: Conjunto de usuarios invitados al cuarto
pub struct Room {
    users: HashMap<usize, String>,
    room_name: String,
    invited: HashSet<String>,
}

impl Room {
    /// Construye un nuevo cuarto con el nombre dado.
    ///
    /// Inicialmente el cuarto no tiene usuarios ni invitados
    pub fn new(room_name: String) -> Self {
        Self {
            users: HashMap::new(),
            room_name,
            invited: HashSet::new(),
        }
    }

    /// Verifica si un usuario está en el cuarto.
    ///
    /// - `username`: El nombre del usuario que se desea verificar si está el cuarto.
    ///
    /// Regresa true si el usuario dado está en el cuarto, false en caso contrario.
    pub fn is_in(&self, username: &str) -> bool {
        self.users.values().any(|u| u == username)
    }

    /// Agrega un usuario al cuarto
    ///
    /// - `username`: El nombre del usuario que se agregará al cuarto.
    /// - `id`: El identificador único del usuario que se agregará al cuarto.
    pub fn add_user(&mut self, username: &str, id: &usize) {
        self.users.insert(*id, username.to_string());
    }

    /// Elimina a un usuario del cuarto
    ///
    /// - `id`: El identificador del usuario que se desea eliminar del cuarto.
    ///
    /// Regresa true si el usuario fue eliminado con éxito,
    /// false si el usuario no estaba en el cuarto.
    pub fn remove_user(&mut self, id: &usize) -> bool {
        self.users.remove(id).is_some()
    }

    /// Regresa un diccionario con los usuarios del cuarto.
    pub fn get_users(&self) -> HashMap<usize, String> {
        self.users.clone()
    }

    /// Regresa el nombre del cuarto.
    pub fn get_room_name(&self) -> String {
        self.room_name.clone()
    }

    /// Verifica si un usuario está en la lista de invitados
    ///
    /// - `username`: El nombre del usuario que se desea verificar si
    /// está en la lista de invitados.
    ///
    /// Regresa true si el usuario está en la lista de invitados, false en caso contrario.
    pub fn is_invited(&self, username: &str) -> bool {
        self.invited.contains(username)
    }

    /// Agrega a un usuario a la lista de invitados
    ///
    /// - `username`: El nombre del usuario que se desea agregar a la lista de invitados.
    pub fn invite_user(&mut self, username: &str) {
        self.invited.insert(username.to_string());
    }
}

#[cfg(test)]
mod tests {}
