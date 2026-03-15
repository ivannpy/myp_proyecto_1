use std::collections::HashSet;
use protocol::status::user::UserStatus;

///
/// Longitud máxima permitida para los nombres de lo usuarios
///
pub const MAX_USERNAME_LEN: usize = 8;

///
/// Valida que el nombre del usuario dado tenga a lo más MAX_USERNAME_LEN caracteres.
///
pub fn validate_username(user: &User) -> bool {
    user.username.len() <= MAX_USERNAME_LEN
}

///
/// Representa a un usuario en el servidor
///
pub struct User {
    username: String,
    status: UserStatus,
    id: usize,
    rooms: HashSet<String>,
    invitations: HashSet<String>,
}

impl User {
    pub fn new(
        username: String,
        status: UserStatus,
        id: usize,
        rooms: HashSet<String>,
        invitations: HashSet<String>,
    ) -> Self {
        Self {
            username,
            status,
            id,
            rooms,
            invitations,
        }
    }

    pub fn set_status(&mut self, new_status: UserStatus) {
        self.status = new_status;
    }

    pub fn get_id(&self) -> usize {
        self.id
    }

    pub fn get_username(&self) -> String {
        self.username.clone()
    }

    pub fn get_status(&self) -> UserStatus {
        self.status.clone()
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_user_username() {
        let user_1 = User {
            username: "12345678".to_string(),
            status: UserStatus::Active,
            id: 0,
            rooms: HashSet::new(),
            invitations: HashSet::new(),
        };
        let user_2 = User {
            username: "123456789".to_string(),
            status: UserStatus::Active,
            id: 1,
            rooms: HashSet::new(),
            invitations: HashSet::new(),
        };

        assert_eq!(validate_username(&user_1), true);
        assert_eq!(validate_username(&user_2), false);
    }
}
