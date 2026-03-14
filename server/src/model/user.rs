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
    state: UserStatus,
    id: usize,
}

impl User {
    pub fn new(username: String, state: UserStatus, id: usize) -> Self {
        Self {
            username,
            state,
            id,
        }   
    }
    
    pub fn set_state(&mut self, new_state: UserStatus) {
        self.state = new_state;
    }

    pub fn get_id(&self) -> usize {
        self.id.clone()
    }
    
    pub fn get_username(&self) -> String {
        self.username.clone()
    }
    
    pub fn get_state(&self) -> UserStatus {
        self.state.clone()
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_user_username() {
        let user_1 = User {
            username: "12345678".to_string(),
            state: UserStatus::Active,
            id: 0,
        };
        let user_2 = User {
            username: "123456789".to_string(),
            state: UserStatus::Active,
            id: 1,
        };

        assert_eq!(validate_username(&user_1), true);
        assert_eq!(validate_username(&user_2), false);
    }
}
