use crate::model::connection_state::ConnectionState;
use crate::model::message::ChatMessage;
use crate::model::room::RemoteRoom;
use crate::model::users::RemoteUser;
use protocol::status::user::UserStatus;
use std::collections::HashMap;

/// Lo que debe saber hacer cualquier vista del chat
///
pub trait ChatView {
    /// Actualiza el estado del chat del usuario local en pantalla
    ///
    /// - `state`: El estado de la conexión del cliente.
    fn update_connection_state(&mut self, state: &ConnectionState);

    /// Actualiza la lista de usuarios conectados en pantalla
    ///
    /// - `user`: Los usuarios conectados en el chat
    fn update_user_list(&mut self, users: &HashMap<String, RemoteUser>);

    /// Actualiza la info del usuario remoto en pantalla
    ///
    /// - `username`:
    /// - `user`:
    fn update_user(&mut self, username: &str, user: &RemoteUser);

    /// Muestra a un usuario a la lista de usuarios conectados en pantalla
    ///
    /// - `user`:
    fn add_user(&mut self, user: &RemoteUser);

    /// Elimina de la pantalla a un usuario que se desconectó
    ///
    /// - `username`:
    fn remove_user(&mut self, username: &str);

    /// Muestra un mensaje recibido en pantalla
    ///
    /// - `message`:
    fn show_message(&mut self, message: &ChatMessage);

    /// Actualiza la info de un cuarto en pantalla
    ///
    /// - `roomname`:
    /// - `room`:
    fn update_room(&mut self, roomname: &str, room: &RemoteRoom);

    /// Agrega un nuevo cuarto en pantalla
    ///
    /// - `room`:
    fn add_room(&mut self, room: &RemoteRoom);

    /// Muestra la lista de usuarios de un cuarto en pantalla
    ///
    /// - `roomname`:
    /// - `members`:
    fn show_room_members(&mut self, roomname: &str, members: &HashMap<String, UserStatus>);

    /// Muestra un error en pantalla
    ///
    /// - `error`:
    fn show_error(&mut self, error: &str);

    /// Muestra una notificación en pantalla
    ///
    /// - `notification`:
    fn show_notification(&mut self, notification: &str);

    /// Pide al usuario que ingrese su nombre de usuario
    fn ask_for_username(&mut self) -> String;

    /// Pide la información del servidor para conectarse
    fn ask_for_server_info(&mut self) -> (String, String);
}
