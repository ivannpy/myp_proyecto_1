use serde::{Deserialize, Serialize};

///
/// Posibles resultados de las operaciones que realiza el servidor
///
#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Result {
    Success,
    UserAlreadyExists,
    NoSuchUser,
    RoomAlreadyExists,
    NoSuchRoom,
    NotInvited,
    NotJoined,
    NotIdentified,
    Invalid,
}

///
/// Operaciones que realiza el servidor a petición de un cliente
///
#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Operation {
    Identify,
    Text,
    NewRoom,
    Invite,
    JoinRoom,
    RoomUsers,
    RoomText,
    LeaveRoom,
    Invalid,
}
