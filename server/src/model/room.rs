use std::collections::{HashMap};

///
/// Modela un cuarto dentro del servidor
///
pub struct Room {
    users: HashMap<usize, String>,
    room_name: String,
    invited: HashMap<usize, String>,
}

impl Room {
    ///
    /// Construye un nuevo cuarto con el nombre dado.
    ///
    pub fn new(room_name: String) -> Self {
        Self {
            users: HashMap::new(),
            room_name,
            invited: HashMap::new(),
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
}

#[cfg(test)]
mod tests {}
