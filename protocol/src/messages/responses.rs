use serde::{Deserialize, Serialize};

/*
   Tipos de respuestas del servidor.
*/
#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ResponseResult {
    Success,
    UserAlreadyExists,
}
