use serde::{Deserialize, Serialize};

pub const MAX_USERNAME_LEN: usize = 8;

pub fn validate_username(username: &str) -> bool {
    username.len() <= MAX_USERNAME_LEN
}

/*
   Representa un usuario activo en el servidor.
*/
pub struct User {
    pub username: String,
    pub state: UserState,
    pub id: usize,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum UserState {
    Active,
    Away,
    Busy,
}
