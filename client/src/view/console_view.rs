use crate::model::connection_state::ConnectionState;
use crate::model::message::ChatMessage;
use crate::model::room::RemoteRoom;
use crate::model::users::RemoteUser;
use crate::view::chat_view::ChatView;
use protocol::status::user::UserStatus;
use std::collections::HashMap;
use std::io;
use std::io::Write;

/// Vista en la terminal
pub struct ConsoleView;

impl ConsoleView {
    pub fn new() -> Self {
        Self {}
    }

    /// Lee de la entrada estándar.
    /// Regresa la cadena leída
    fn read_from_stdin(&mut self) -> String {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        input.trim().to_string()
    }

    /// Escribe en la salida estándar.
    ///
    /// - `text`: El texto a escribir
    fn write_to_stdout(&mut self, text: String) {
        print!("{}", text);
        io::stdout().flush().unwrap();
    }

    /// Muestra en la pantalla el texto dado.
    ///
    /// - `text`: El texto a mostrar en la pantalla.
    fn show_in_screen(&mut self, text: String) {
        self.write_to_stdout(text);
    }
}

impl ChatView for ConsoleView {
    fn update_connection_state(&mut self, state: &ConnectionState) {
        self.show_in_screen(format!("Estado de conexión: {:?}", state));
    }

    fn update_user_list(&mut self, users: &HashMap<String, RemoteUser>) {
        self.show_in_screen("=== Usuarios conectados ===".to_string());

        for (username, user) in users {
            self.show_in_screen(format!("  {} ({:?})", username, user.get_status()));
        }
    }

    fn update_user(&mut self, username: &str, user: &RemoteUser) {
        self.show_in_screen(format!(
            "EL usuario {} actualizó su estado: {:?}",
            username,
            user.get_status()
        ));
    }

    fn add_user(&mut self, user: &RemoteUser) {
        self.show_in_screen(format!(
            "Nuevo usuario conectado: {} ({:?})",
            user.get_username(),
            user.get_status()
        ));
    }

    fn remove_user(&mut self, username: &str) {
        self.show_in_screen(format!("Usuario desconectado: {}", username));
    }

    fn show_message(&mut self, message: &ChatMessage) {
        match message {
            ChatMessage::Private { from, text, .. } => {
                self.show_in_screen(format!("[Privado de {}] {}", from, text));
            }
            ChatMessage::Public { from, text, .. } => {
                self.show_in_screen(format!("[Público de {}] {}", from, text));
            }
            ChatMessage::Room {
                roomname,
                from,
                text,
                ..
            } => {
                self.show_in_screen(format!("[{} en {}] {}", from, roomname, text));
            }
        }
    }

    fn update_room(&mut self, roomname: &str, room: &RemoteRoom) {
        self.show_in_screen(format!("Nuevos usuarios en el cuarto {}", roomname));
        for (username, status) in room.get_users() {
            self.show_in_screen(format!("  {} ({:?})", username, status));
        }
    }

    // Agrega un cuarto.
    fn add_room(&mut self, room: &RemoteRoom) {
        self.show_in_screen(format!(
            "Has sido incluido al cuarto: {}",
            room.get_roomname()
        ));
    }

    fn show_room_members(&mut self, _roomname: &str, members: &HashMap<String, UserStatus>) {
        self.show_in_screen(format!(
            "=== Lista de usuarios en el cuarto {} ===",
            _roomname
        ));

        for (username, status) in members {
            self.show_in_screen(format!("  {} ({:?})", username, status));
        }
    }

    fn show_error(&mut self, _error: &str) {
        self.show_in_screen(format!("Error: {}", _error));
    }

    fn show_notification(&mut self, notification: &str) {
        self.show_in_screen(format!("Notificación {}", notification));
    }

    fn ask_for_username(&mut self) -> String {
        let mut username = String::new();
        self.show_in_screen("Ingresa tu nombre de usuario:".to_string());
        loop {
            username.clear();
            self.show_in_screen(">".to_string());
            match io::stdin().read_line(&mut username) {
                Ok(_) => {
                    if username.trim().is_empty() {
                        self.show_in_screen(
                            "El nombre de usuario no puede estar vacío. Inténtalo de nuevo."
                                .to_string(),
                        );
                        continue;
                    }
                    username = username.trim().to_string();
                    break;
                }
                Err(_) => {
                    self.show_in_screen("Error al leer. Inténtalo de nuevo.".to_string());
                }
            }
        }
        username
    }

    fn ask_for_server_info(&mut self) -> (String, String) {
        self.show_in_screen("Dirección del servidor".to_string());
        let mut addr = String::new();

        loop {
            addr.clear();
            self.show_in_screen(">".to_string());
            addr = self.read_from_stdin();
            if addr.is_empty() {
                self.show_in_screen("No puede estar vacío.".to_string());
                continue;
            }
            break;
        }

        self.show_in_screen("Puerto".to_string());
        let mut port = String::new();

        loop {
            port.clear();
            self.show_in_screen(">".to_string());
            port = self.read_from_stdin();
            if port.is_empty() {
                self.show_in_screen("No puede estar vacío.".to_string());
                continue;
            }
            break;
        }

        (addr, port)
    }
}
