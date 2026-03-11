use crate::model::user::User;
use std::collections::HashMap;
use std::sync::atomic::{AtomicUsize, Ordering};

/*
   Estado del servidor.
*/
pub struct ServerState {
    users: HashMap<String, User>,
    conn_counter: AtomicUsize,
}

impl ServerState {
    /*
       Crea un nuevo estado del servidor.
    */
    pub fn new() -> Self {
        Self {
            users: HashMap::new(),
            conn_counter: AtomicUsize::new(0),
        }
    }

    /*
       Regresa el siguiente id disponible.
    */
    pub fn get_next_id(&self) -> usize {
        self.conn_counter.fetch_add(1, Ordering::SeqCst)
    }

    /*
       Regresa los usuarios activos.
    */
    pub fn get_users(&self) -> &HashMap<String, User> {
        &self.users
    }

    /*
       Agrega un nuevo usuario a los usuarios activos.
    */
    pub fn insert_user(&mut self, user: User) {
        self.users.insert(user.username.clone(), user);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::mpsc;

    #[test]
    fn test_server_init_state() {
        let state = ServerState::new();
        assert_eq!(state.get_users().len(), 0);
        assert_eq!(state.get_next_id(), 0);
    }

    #[test]
    fn test_server_insert_user() {
        let mut state = ServerState::new();
        let user = User {
            username: "user_1".to_string(),
            id: state.get_next_id(),
            sender: mpsc::channel().0,
        };
        state.insert_user(user);
        assert_eq!(state.get_users().len(), 1);
    }

    #[test]
    fn test_counter_after_insert() {
        let mut state = ServerState::new();
        let user = User {
            username: "user_1".to_string(),
            id: state.get_next_id(),
            sender: mpsc::channel().0,
        };
        state.insert_user(user);
        assert_eq!(state.get_next_id(), 1);
    }
}
