use serde::{Deserialize, Serialize};

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
    pub state: UserState,
    pub id: usize,
}

///
/// Representa el estado de un usuario en el servidor.
///
#[derive(Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum UserState {
    Active,
    Away,
    Busy,
}

mod tests {
    use super::*;

    #[test]
    fn test_user_username() {
        let user_1 = User {
            username: "12345678".to_string(),
            state: UserState::Active,
            id: 0,
        };
        let user_2 = User {
            username: "123456789".to_string(),
            state: UserState::Active,
            id: 1,
        };

        assert_eq!(validate_username(&user_1), true);
        assert_eq!(validate_username(&user_2), false);
    }
}
