use serde::{Deserialize, Serialize};

/*
   Tipos de respuestas del servidor.
*/
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ResponseResult {
    Success,
    UserAlreadyExists,
}
