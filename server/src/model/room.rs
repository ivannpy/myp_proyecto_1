use std::collections::{HashMap, HashSet};

pub const MAX_ROOM_NAME_LEN: usize = 16;

pub fn validate_room_name(room: &str) -> bool {
    room.len() <= MAX_ROOM_NAME_LEN
}

///
/// Modela un cuarto dentro del servidor
///
pub struct Room {
    users: HashMap<usize, String>,
    room_name: String,
    invited: HashSet<String>,
}

impl Room {
    ///
    /// Construye un nuevo cuarto con el nombre dado.
    ///
    pub fn new(room_name: String) -> Self {
        Self {
            users: HashMap::new(),
            room_name,
            invited: HashSet::new(),
        }
    }

    ///
    /// Verifica si un usuario está en el cuarto.
    ///
    pub fn is_in(&self, username: &str) -> bool {
        self.users.values().any(|u| u == username)
    }

    ///
    /// Agrega un usuario al cuarto
    ///
    pub fn add_user(&mut self, username: &str, id: usize) {
        self.users.insert(id, username.to_string());
    }

    ///
    /// Elimina a un usuario del cuarto
    ///
    pub fn remove_user(&mut self, id: &usize) -> bool {
        self.users.remove(id).is_some()
    }

    ///
    /// Regresa una lista con los nombres de usuario de los usuarios en el cuarto
    ///
    pub fn get_users(&self) -> HashMap<usize, String> {
        self.users.clone()
    }

    pub fn get_room_name(&self) -> String {
        self.room_name.clone()
    }

    ///
    /// Verifica si un username está en los invitados
    ///
    pub fn is_invited(&self, username: &str) -> bool {
        self.invited.contains(username)
    }
    
    ///
    /// Agrega a un usuario a la lista de invitados
    /// 
    pub fn invite_user(&mut self, username: &str) {
        self.invited.insert(username.to_string());   
    }
}

#[cfg(test)]
mod tests {}
