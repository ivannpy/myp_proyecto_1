/*
    Los mensajes que recibe el cliente.
 */
pub enum ClientMessage {
    Response {
        operation: String,
        result: String,
        extra: String,
    },
}
