use crate::model::connection_state::ConnectionState;
use crate::model::message::ChatMessage;
use crate::model::room::RemoteRoom;
use crate::model::users::RemoteUser;
use protocol::status::user::UserStatus;
use std::collections::HashMap;

///
/// Lo que debe saber hacer cualquier vista del chat
/// 
pub trait ChatView {
    ///
    /// Actualiza el estado del chat del usuario local en pantalla
    /// 
    fn update_connection_state(&mut self, state: &ConnectionState);

    ///
    /// Actualiza la lista de usuarios conectados en pantalla
    /// 
    fn update_user_list(&mut self, users: &HashMap<String, RemoteUser>);

    ///
    /// Actualiza la info del usuario remoto en pantalla
    /// 
    fn update_user(&mut self, username: &str, user: &RemoteUser);

    ///
    /// Muestra a un usuario a la lista de usuarios conectados en pantalla
    /// 
    fn add_user(&mut self, user: &RemoteUser);

    ///
    /// Elimina de la pantalla a un usuario que se desconectó 
    /// 
    fn remove_user(&mut self, username: &str);

    ///
    /// Muestra un mensaje recibido en pantalla
    /// 
    fn show_message(&mut self, message: &ChatMessage);

    ///
    /// Actualiza la lista de cuartos en pantalla
    /// 
    fn update_room_list(&mut self, rooms: &HashMap<String, RemoteRoom>);

    ///
    /// Actualiza la info de un cuarto en pantalla 
    /// 
    fn update_room(&mut self, roomname: &str, room: &RemoteRoom);

    ///
    /// Agrega un nuevo cuarto en pantalla
    /// 
    fn add_room(&mut self, room: &RemoteRoom);

    ///
    /// Muestra la lista de usuarios de un cuarto en pantalla 
    /// 
    fn show_room_members(&mut self, roomname: &str, members: &HashMap<String, UserStatus>);

    ///
    /// Muestra un error en pantalla
    /// 
    fn show_error(&mut self, error: &str);

    ///
    /// Muestra una notificación en pantalla
    fn show_notification(&mut self, notification: &str);
}
