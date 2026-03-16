use crate::messages::responses::{Operation, OperationResult};
use crate::status::user::UserStatus;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Mensajes que el servidor envía al cliente.
///
#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(tag = "type", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ClientMessage {
    /// La respuesta del servidor a una operación solicitada por el cliente
    ///
    /// - `operation`: El tipo de operación que se realizó.
    /// - `result`: El resultado de la operación.
    /// - `extra`: Información adicional sobre la operación.
    Response {
        operation: Operation,
        result: OperationResult,
        extra: Option<String>,
    },
    /// Se recibe cuando un nuevo usuario se conecta e identifica correctamente.
    ///
    /// - `username`: El nombre del usuario que se conectó al chat
    NewUser { username: String },
    /// Se recibe cuando un usuario cambia su status.
    ///
    /// - `username`: El nombre del usuario que cambió su status.
    /// - `status`: El nuevo status del usuario.
    NewStatus {
        username: String,
        status: UserStatus,
    },
    /// Se recibe cuando el cliente solicita al servidor la lista de usuarios conectados al chat.
    ///
    /// - `users`: La lista de usuarios conectados al chat con sus status.
    UserList { users: HashMap<String, UserStatus> },
    /// Se recibe cuando un usuario remoto envía un mensaje privado.
    ///
    /// - `username`: El nombre del usuario que mandó el mensaje.
    /// - `text`: El texto del mensaje.
    TextFrom { username: String, text: String },
    /// Se recibe cuando un usuario envía un mensaje público.
    ///
    /// - `username`: El nombre del usuairo que mandó el mensaje público.
    /// - `text`: El texto del mensaje público.
    PublicTextFrom { username: String, text: String },
    /// Se recibe cuando un usuario remoto invita al usuario local a unirse a un cuarto.
    ///
    /// - `roomname`: El nombre del cuarto al que fue invitado.
    /// - `username`: El usuario que invito al usuario local.
    Invitation { roomname: String, username: String },
    /// Se recibe cuando un usuario remoto se une a un cuarto.
    ///
    /// - `roomname`: El nombre del cuarto al que se unió el usuario remoto.
    /// - `username`: El nombre del usuario que se unió al cuarto.
    JoinedRoom { roomname: String, username: String },
    /// Se recibe en respuesta a una solicitud de lista de usuarios de un cuarto.
    ///
    /// - `roomname`: El nombre del cuarto.
    /// - `users`: La lista de usuarios conectados al cuarto con sus status.
    RoomUserList {
        roomname: String,
        users: HashMap<String, UserStatus>,
    },
    /// Se recibe cuando llega un mensaje de un usuario a un cuarto.
    ///
    /// - `username`: El nombre del usuario que envió el mensaje.
    /// - `roomname`: El nombre del cuarto al que se envió el mensaje.
    /// - `text`: El texto del mensaje.
    RoomTextFrom {
        roomname: String,
        username: String,
        text: String,
    },
    /// Se recibe cuando un usuario remoto se deja un cuarto.
    ///
    /// - `username`: El usuario que salió del cuarto.
    /// - `roomname`: El cuarto del que salió.
    LeftRoom { roomname: String, username: String },
    /// Se recibe cuando un usuario se desconecta del chat.
    ///
    /// - `username`: El usuario que se desconectó del chat.
    Disconnected { username: String },
}
