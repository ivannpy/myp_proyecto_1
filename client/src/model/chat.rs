use crate::model::connection_state::ConnectionState;
use crate::model::message::ChatMessage;
use crate::model::room::RemoteRoom;
use crate::model::users::{LocalUser, RemoteUser};
use protocol::status::user::UserStatus;
use std::collections::HashMap;

pub struct Chat {
    local_user: LocalUser,
    remote_users: HashMap<String, RemoteUser>,
    rooms: HashMap<String, RemoteRoom>,
    messages: Vec<ChatMessage>,
    connection_state: ConnectionState,
}

impl Chat {
    pub fn new(local_user: LocalUser) -> Self {
        Self {
            local_user,
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
}
