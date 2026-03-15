use protocol::status::user::UserStatus;
use std::collections::HashMap;

pub struct RemoteRoom {
    room_name: String,
    users: HashMap<String, UserStatus>,
    is_invited: bool,
    is_joined: bool,
}

impl RemoteRoom {
    pub fn new(room_name: String, is_invited: bool, is_joined: bool) -> Self {
        Self {
            room_name,
            users: HashMap::new(),
            is_invited,
            is_joined,
        }
    }
    pub fn set_is_invited(&mut self, is_invited: bool) {
        self.is_invited = is_invited;
    }
    pub fn set_is_joined(&mut self, is_joined: bool) {
        self.is_joined = is_joined;
    }

    pub fn set_users(&mut self, users: HashMap<String, UserStatus>) {
        self.users = users;
    }
    pub fn get_users(&self) -> &HashMap<String, UserStatus> {
        &self.users
    }

    pub fn add_new_user(&mut self, username: String, status: UserStatus) {
        self.users.insert(username, status);
    }
    pub fn remove_user(&mut self, username: &str) {
        self.users.remove(username);
    }
    pub fn is_in(&self, username: &str) -> bool {
        self.users.contains_key(username)
    }
}
