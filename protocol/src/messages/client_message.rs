use crate::messages::responses::ResponseResult;
use serde::{Deserialize, Serialize};

/*
   Los mensajes que recibe el cliente.
*/
#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "type", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ClientMessage {
    Response {
        operation: String,
        result: ResponseResult,
        extra: String,
    },
}
