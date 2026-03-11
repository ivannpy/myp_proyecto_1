use crate::status::user::UserStatus;
use serde::{Deserialize, Serialize};

/*
   Los mensajes que recibe el servidor.
*/
#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(tag = "type", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ServerMessage {
    Identify {
        username: String,
    },
    Status {
        status: UserStatus,
    },
    Users,
    Text {
        username: String,
        text: String,
    },
    PublicText {
        text: String,
    },
    NewRoom {
        roomname: String,
    },
    Invite {
        roomname: String,
        usernames: Vec<String>,
    },
    JoinRoom {
        roomname: String,
    },
    RoomUsers {
        roomname: String,
    },
    RoomText {
        roomname: String,
        text: String,
    },
    LeaveRoom {
        roomname: String,
    },
    Disconnect,
}
