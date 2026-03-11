use crate::messages::responses::{Operation, Result};
use crate::status::user::UserStatus;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/*
   Los mensajes que recibe el cliente.
*/
#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(tag = "type", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ClientMessage {
    Response {
        operation: Operation,
        result: Result,
        extra: Option<String>,
    },
    NewUser {
        username: String,
    },
    NewStatus {
        username: String,
        status: UserStatus,
    },
    UserList {
        users: HashMap<String, UserStatus>,
    },
    TextFrom {
        username: String,
        text: String,
    },
    PublicTextFrom {
        username: String,
        text: String,
    },
    Invitation {
        roomname: String,
        username: String,
    },
    JoinedRoom {
        roomname: String,
        username: String,
    },
    RoomUserList {
        roomname: String,
        users: HashMap<String, UserStatus>,
    },
    RoomTextFrom {
        roomname: String,
        username: String,
        text: String,
    },
    LeftRoom {
        roomname: String,
        username: String,
    },
    Disconnected {
        username: String,
    },
}
