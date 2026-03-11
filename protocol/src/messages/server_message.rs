use serde::{Deserialize, Serialize};

/*
   Los mensajes que recibe el servidor.
*/
#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "type", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ServerMessage {
    Identify { username: String },
}
