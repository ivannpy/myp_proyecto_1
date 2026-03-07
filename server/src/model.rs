use std::collections::HashMap;
use std::sync::mpsc;

pub struct User {
    pub username: String,
    pub id: String,
}

pub struct ServerState {
    pub connections: HashMap<String, mpsc::Sender<String>>,
}