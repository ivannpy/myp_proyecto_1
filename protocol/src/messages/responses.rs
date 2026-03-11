use serde::{Deserialize, Serialize};

/*
   Tipos de respuestas del servidor.
*/
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
