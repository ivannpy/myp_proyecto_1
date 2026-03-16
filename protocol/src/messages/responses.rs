use serde::{Deserialize, Serialize};

/// Resultados de las operaciones que pide el cliente.
///
#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OperationResult {
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

/// Operaciones que el cliente pide al servidor.
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
