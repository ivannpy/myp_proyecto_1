use std::collections::HashMap;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::mpsc;

pub struct User {
    pub username: String,
    pub id: usize,
    pub sender: mpsc::Sender<String>,
}

pub struct ServerState {
    users: HashMap<String, User>,
    conn_counter: AtomicUsize,
}

impl ServerState {
    pub fn new() -> Self {
        Self {
            users: HashMap::new(),
            conn_counter: AtomicUsize::new(0),
        }
    }
    
    pub fn get_next_id(&self) -> usize {
        self.conn_counter.fetch_add(1, Ordering::SeqCst)
    }
    
    pub fn get_users(&self) -> &HashMap<String, User> {
        &self.users
    }
    
    pub fn insert_user(&mut self, user: User) {
        self.users.insert(user.username.clone(), user);
    }
}
