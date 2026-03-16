use protocol::status::user::UserStatus;

#[derive(Debug, Clone)]
pub struct LocalUser {
    username: String,
    status: UserStatus,
    connected: bool,
}

impl LocalUser {
    pub fn new(username: String, status: UserStatus) -> Self {
        Self {
            username,
            status,
            connected: false,
        }
    }

    pub fn set_connected(&mut self, connected: bool) {
        self.connected = connected;
    }
    pub fn is_connected(&self) -> bool {
        self.connected
    }
    pub fn get_username(&self) -> &str {
        &self.username
    }
    pub fn set_username(&mut self, username: String) {
        self.username = username;
    }

    pub fn get_status(&self) -> &UserStatus {
        &self.status
    }
    pub fn set_status(&mut self, status: UserStatus) {
        self.status = status;
    }
}

#[derive(Debug, Clone)]
pub struct RemoteUser {
    username: String,
    status: UserStatus,
}
impl RemoteUser {
    pub fn new(username: String, status: UserStatus) -> Self {
        Self { username, status }
    }
    pub fn get_username(&self) -> &str {
        &self.username
    }
    pub fn get_status(&self) -> &UserStatus {
        &self.status
    }
    pub fn set_status(&mut self, status: UserStatus) {
        self.status = status;
    }
    pub fn set_username(&mut self, username: String) {
        self.username = username;
    }
}
