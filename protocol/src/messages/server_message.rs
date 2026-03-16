use crate::status::user::UserStatus;
use serde::{Deserialize, Serialize};

/// Mensajes que envía el cliente al servidor.
///
#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(tag = "type", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ServerMessage {
    /// El cliente solicita identificarse en el servidor.
    ///
    /// - `username`: Nombre de usuario que el cliente desea utilizar.
    Identify { username: String },
    /// El cliente solicita cambiar su status.
    ///
    /// - `status`: Nuevo status del usuario
    Status { status: UserStatus },
    /// El cliente solicita la lista de usuarios conectados al chat.
    Users,
    /// El cliente solicita enviar un mensaje privado a un usuario.
    ///
    /// - `username`: Nombre de usuario al que se desea enviar el mensaje.
    /// - `text`: Contenido del mensaje privado.
    Text { username: String, text: String },
    /// El cliente solicita enviar un mensaje público.
    ///
    /// - `text`: Contenido del mensaje público.
    PublicText { text: String },
    /// El cliente solcita crear un cuarto.
    ///
    /// - `roomname`: Nombre del cuarto que se desea crear.
    NewRoom { roomname: String },
    /// El cliente solicita invitar a otros usuarios a unirse a un cuarto.
    ///
    /// - `roomname`: El cuarto al que desea invitar.
    /// - `usernames`: Lista de usuarios a los que desea invitar.
    Invite {
        roomname: String,
        usernames: Vec<String>,
    },
    /// El cliente solicita unirse a un cuarto.
    ///
    /// - `roomname`: Nombre del cuarto al que desea unirse.
    JoinRoom { roomname: String },
    /// El cliente solicita la lista de usuarios de un cuarto.
    ///
    /// - `roomname`: Nombre del cuarto del que se desea obtener la lista de usuarios.
    RoomUsers { roomname: String },
    /// El cliente solicita enviar un mensaje a un cuarto.
    ///
    /// - `roomname`: Nombre del cuarto al que se desea enviar un mensaje.
    /// - `text`: Contenido del mensaje.
    RoomText { roomname: String, text: String },
    /// El cliente solicita abandonar un cuarto.
    ///
    /// - `roomname`: Nombre del cuarto del que se desea abandonar.
    LeaveRoom { roomname: String },
    /// El cliente solicita desconectarse del chat.
    Disconnect,
}
