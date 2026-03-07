use std::collections::HashMap;
use std::sync::mpsc;

pub struct User {
    pub id: String,
    pub sender: mpsc::Sender<String>,
}

pub struct ServerState {
    pub users: HashMap<String, User>,
}
