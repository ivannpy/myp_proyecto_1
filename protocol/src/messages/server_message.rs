/*
   Los mensajes que recibe el servidor.
*/
pub enum ServerMessage {
    Identify { username: String },
}
