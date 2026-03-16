use crate::model::connection_state::ConnectionState;
use crate::model::message::ChatMessage;
use crate::model::room::RemoteRoom;
use crate::model::users::{LocalUser, RemoteUser};
use protocol::status::user::UserStatus;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct ChatModel {
    local_user: LocalUser,
    remote_users: HashMap<String, RemoteUser>,
    rooms: HashMap<String, RemoteRoom>,
    messages: Vec<ChatMessage>,
    connection_state: ConnectionState,
}

impl ChatModel {
    pub fn new() -> Self {
        Self {
            local_user: LocalUser::new(String::from(""), UserStatus::Active),
            remote_users: HashMap::new(),
            rooms: HashMap::new(),
            messages: Vec::new(),
            connection_state: ConnectionState::Disconnected,
        }
    }

    pub fn add_remote_user(&mut self, username: String, status: UserStatus) {
        self.remote_users
            .insert(username.clone(), RemoteUser::new(username, status));
    }

    pub fn remove_remote_user(&mut self, username: &str) {
        self.remote_users.remove(username);
    }

    pub fn update_user_status(&mut self, username: &str, status: UserStatus) {
        if let Some(user) = self.remote_users.get_mut(username) {
            user.set_status(status);
        }
    }

    pub fn update_local_user_status(&mut self, status: UserStatus) {
        self.local_user.set_status(status);
    }

    pub fn update_local_user_connected(&mut self, connected: bool) {
        self.local_user.set_connected(connected);
    }

    pub fn add_message(&mut self, message: ChatMessage) {
        self.messages.push(message);
    }

    pub fn add_room(&mut self, roomname: String, is_joined: bool, is_invited: bool) {
        self.rooms.insert(
            roomname.clone(),
            RemoteRoom::new(roomname, is_invited, is_joined),
        );
    }

    pub fn join_room(&mut self, roomname: &str) {
        if let Some(room) = self.rooms.get_mut(roomname) {
            room.set_is_invited(false);
            room.set_is_joined(true);
        }
    }

    pub fn leave_room(&mut self, roomname: &str) {
        if let Some(room) = self.rooms.get_mut(roomname) {
            room.set_is_joined(false);
            room.set_users(HashMap::new());
        }
    }

    pub fn add_room_member(&mut self, roomname: &str, username: String, status: UserStatus) {
        if let Some(room) = self.rooms.get_mut(roomname) {
            room.add_new_user(username, status);
        }
    }

    pub fn remove_room_member(&mut self, roomname: &str, username: &str) {
        if let Some(room) = self.rooms.get_mut(roomname) {
            room.remove_user(username);
        }
    }

    pub fn get_connection_state(&self) -> ConnectionState {
        self.connection_state.clone()
    }

    pub fn set_connection_state(&mut self, state: ConnectionState) {
        self.connection_state = state;
    }

    pub fn get_local_user(&self) -> &LocalUser {
        &self.local_user
    }

    pub fn get_remote_users(&self) -> &HashMap<String, RemoteUser> {
        &self.remote_users
    }

    pub fn get_rooms(&self) -> &HashMap<String, RemoteRoom> {
        &self.rooms
    }

    pub fn get_rooms_mut(&mut self) -> Option<&mut HashMap<String, RemoteRoom>> {
        Some(&mut self.rooms)
    }

    pub fn clean_remote_users(&mut self) {
        self.remote_users.clear();
    }

    pub fn get_messages(&self) -> &Vec<ChatMessage> {
        &self.messages
    }

    pub fn clean_messages(&mut self) {
        self.messages.clear();
    }
    
    pub fn set_local_username(&mut self, username: &str) {
        self.local_user.set_username(username.to_string());
    }
}
