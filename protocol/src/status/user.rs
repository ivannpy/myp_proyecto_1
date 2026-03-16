use serde::{Deserialize, Serialize};

///
/// El status de un usuario en el chat.
///
#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum UserStatus {
    Active,
    Away,
    Busy,
}
