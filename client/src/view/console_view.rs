use crate::model::connection_state::ConnectionState;
use crate::model::message::ChatMessage;
use crate::model::room::RemoteRoom;
use crate::model::users::RemoteUser;
use crate::view::chat_view::ChatView;
use protocol::status::user::UserStatus;
use std::collections::HashMap;

pub struct ConsoleView;

impl ConsoleView {
    pub fn new() -> Self {
        Self {}
    }
}

impl ChatView for ConsoleView {
    fn update_connection_state(&mut self, state: &ConnectionState) {
        println!("Estado de conexión: {:?}", state);
    }

    fn update_user_list(&mut self, users: &HashMap<String, RemoteUser>) {
        println!("=== Usuarios conectados ===");
        for (username, user) in users {
            println!("  {} ({:?})", username, user.get_status());
        }
    }

    fn update_user(&mut self, username: &str, user: &RemoteUser) {
        println!(
            "EL usuario {} actualizó su estado: {:?}",
            username,
            user.get_status()
        );
    }

    fn add_user(&mut self, user: &RemoteUser) {
        println!(
            "Nuevo usuario conectado: {} ({:?})",
            user.get_username(),
            user.get_status()
        );
    }

    fn remove_user(&mut self, username: &str) {
        println!("Usuario desconectado: {}", username);
    }

    fn show_message(&mut self, message: &ChatMessage) {
        match message {
            ChatMessage::Private { from, text, .. } => {
                println!("[Privado de {}] {}", from, text);
            }
            ChatMessage::Public { from, text, .. } => {
                println!("[Público de {}] {}", from, text);
            }
            ChatMessage::Room {
                roomname,
                from,
                text,
                ..
            } => {
                println!("[{} en {}] {}", from, roomname, text);
            }
        }
    }

    fn update_room(&mut self, roomname: &str, room: &RemoteRoom) {
        println!("Nuevos usuarios en el cuarto {}", roomname);
        for (username, status) in room.get_users() {
            println!("  {} ({:?})", username, status);
        }
    }

    // Agrega un cuarto.
    fn add_room(&mut self, room: &RemoteRoom) {
        println!(" Has sido includo al cuarto {}", room.get_roomname())
    }

    fn show_room_members(&mut self, roomname: &str, members: &HashMap<String, UserStatus>) {
        println!("=== Lista de usuarios en el cuarto {} ===", roomname);
        for (username, status) in members {
            println!("  {} ({:?})", username, status);
        }
    }

    fn show_error(&mut self, error: &str) {
        println!("Error: {}", error);
    }

    fn show_notification(&mut self, notification: &str) {
        println!("Notificación: {}", notification);
    }
}
