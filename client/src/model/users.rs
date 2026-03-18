use protocol::status::user::UserStatus;

/// El usuario local
///
/// - `username`: Su nombre de usuario en el chat
/// - `status`: El estado del usuario
/// - `connected`: Si el usuario está conectado al chat
#[derive(Debug, Clone)]
pub struct LocalUser {
    username: String,
    status: UserStatus,
    connected: bool,
}

impl LocalUser {
    /// Crea un nuevo usuario. Inicialmente está desconectado.
    pub fn new(username: String, status: UserStatus) -> Self {
        Self {
            username,
            status,
            connected: false,
        }
    }

    /// Fija el estado de conexión del usuario local
    pub fn set_connected(&mut self, connected: bool) {
        self.connected = connected;
    }

    /// Regresa el nombre de usuario del usuario local
    pub fn get_username(&self) -> &str {
        &self.username
    }

    /// Fija el nombre de usuario del usuario local
    pub fn set_username(&mut self, username: String) {
        self.username = username;
    }

    /// Fija el estado en el chat del usuario local
    pub fn set_status(&mut self, status: UserStatus) {
        self.status = status;
    }
}

/// Un usuario remoto
///
/// - `username`: Su nombre de usuario
/// - `status`: Su estado en el chat
#[derive(Debug, Clone)]
pub struct RemoteUser {
    username: String,
    status: UserStatus,
}

impl RemoteUser {
    /// Cre un nuevo usuario remoto
    pub fn new(username: String, status: UserStatus) -> Self {
        Self { username, status }
    }

    /// Regresa el nombre de usuario de un usuario remoto
    pub fn get_username(&self) -> &str {
        &self.username
    }

    /// Regresa el estado en el chat de un usuario remoto
    pub fn get_status(&self) -> &UserStatus {
        &self.status
    }

    /// Fija el estado en el chat de un usuario remoto.
    pub fn set_status(&mut self, status: UserStatus) {
        self.status = status;
    }
}
