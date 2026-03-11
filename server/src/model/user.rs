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
    pub username: String,
    pub state: UserStatus,
    pub id: usize,
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
