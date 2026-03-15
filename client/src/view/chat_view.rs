use crate::model::connection_state::ConnectionState;
use crate::model::message::ChatMessage;
use crate::model::room::RemoteRoom;
use crate::model::users::RemoteUser;
use protocol::status::user::UserStatus;
use std::collections::HashMap;

pub trait ChatView {
    // Actualizar estado de conexión
    fn update_connection_state(&mut self, state: &ConnectionState);

    // Actualizar lista de usuarios
    fn update_user_list(&mut self, users: &HashMap<String, RemoteUser>);

    // Actualizar usuario específico
    fn update_user(&mut self, username: &str, user: &RemoteUser);

    // Agregar usuario a la lista
    fn add_user(&mut self, user: &RemoteUser);

    // Remover usuario de la lista
    fn remove_user(&mut self, username: &str);

    // Mostrar mensaje
    fn show_message(&mut self, message: &ChatMessage);

    // Actualizar lista de cuartos
    fn update_room_list(&mut self, rooms: &HashMap<String, RemoteRoom>);

    // Actualizar cuarto específico
    fn update_room(&mut self, roomname: &str, room: &RemoteRoom);

    // Mostrar miembros de un cuarto
    fn show_room_members(&mut self, roomname: &str, members: &HashMap<String, UserStatus>);

    // Mostrar error
    fn show_error(&mut self, error: &str);

    // Mostrar notificación
    fn show_notification(&mut self, notification: &str);
}
