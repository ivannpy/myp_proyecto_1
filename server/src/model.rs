use std::collections::HashMap;
use crate::connection::Connection;

pub struct User {
    pub username: String,
    pub id: String,
}

pub struct ServerState {
    pub connections: HashMap<String, Connection>,
}