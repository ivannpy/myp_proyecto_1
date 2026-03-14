use std::collections::{HashMap, LinkedList};

///
/// Modela un cuarto dentro del servidor
///
pub struct Room {
    users: HashMap<usize, String>,
    room_name: String,
}

impl Room {
    ///
    /// Construye un nuevo cuarto con el nombre dado.
    ///
    pub fn new(room_name: String) -> Self {
        Self {
            users: HashMap::new(),
            room_name,
        }
    }

    ///
    /// Verifica si un usuario está en el cuarto.
    ///
    pub fn is_in(&self, user_id: usize) -> bool {
        self.users.contains_key(&user_id)
    }

    ///
    /// Agrega un usuario al cuarto
    ///
    pub fn add_user(&mut self, user_id: usize, username: String) {
        self.users.insert(user_id, username);
    }

    ///
    /// Elimina a un usuario del cuarto
    ///
    pub fn remove_user(&mut self, user_id: usize) {
        self.users.remove(&user_id);
    }

    ///
    /// Regresa una lista con los nombres de usuario de los usuarios en el cuarto
    ///
    pub fn get_user_list(&self) -> LinkedList<String> {
        self.users
            .iter()
            .map(|(_, username)| username.clone())
            .collect::<LinkedList<String>>()
    }
}

#[cfg(test)]
mod tests {}
