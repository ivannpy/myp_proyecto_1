use serde::{Deserialize, Serialize};

///
/// Representa el estado de un usuario en el servidor.
///
#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum UserStatus {
    Active,
    Away,
    Busy,
}
