use crate::client_error::ClientError;
use crate::model::chat_model::ChatModel;
use crate::model::connection_state::ConnectionState;
use crate::model::message::ChatMessage;
use crate::network_connection::{NetworkReader, NetworkWriter, connect};
use crate::view::chat_view::ChatView;
use crate::view::console_view::ConsoleView;
use protocol::messages::client_message::ClientMessage;
use protocol::messages::responses::{Operation, OperationResult};
use protocol::messages::server_message::ServerMessage;
use protocol::status::user::UserStatus;
use std::collections::HashMap;
use std::io;
use std::io::{BufRead, Write};
use std::sync::{Arc, Mutex, mpsc};

pub type MessageSender = mpsc::Sender<ServerMessage>;

/// Controlador
///
/// - `model`: El modelo del chat
/// - `view`: Una vista
/// - `writer`: Para mandarle mensajes al servidor
/// - `reader`: Para recibir mensajes del servidor
pub struct ChatController<V: ChatView> {
    pub model: ChatModel,
    pub view: V,
    pub sender: Option<MessageSender>,
    pub reader: Option<NetworkReader>,
    pub writer: Option<NetworkWriter>,
}

impl<V: ChatView> ChatController<V> {
    /// Crea un nuevo controlador para el chat
    pub fn new(view: V) -> Self {
        Self {
            model: ChatModel::new(),
            view,
            sender: None,
            reader: None,
            writer: None,
        }
    }

    /// Verifica si el usuario está identificado
    pub fn is_identified(&self) -> bool {
        matches!(
            self.model.get_connection_state(),
            ConnectionState::Identified
        )
    }

    /// Establece el canal de envío
    pub fn set_sender(&mut self, sender: MessageSender) {
        self.sender = Some(sender);
    }

    ///
    /// Implementa la funcionalidad protocolo
    ///

    /// Maneja la identificación del usuario local con el servidor
    pub fn identify(&mut self) -> Result<(), ClientError> {
        let username = self.ask_for_username();
        let msg_to_send = ServerMessage::Identify { username };

        if let Some(sender) = &self.sender {
            sender
                .send(msg_to_send)
                .map_err(|_| ClientError::ConnectionError)?;
        } else {
            return Err(ClientError::ConnectionError);
        }
        Ok(())
    }

    ///
    /// Envío de solicitudes
    ///

    /// Cambiar estado en el chat del usuario local
    ///
    /// - `status`: El nuevo estado del usuario local
    pub fn set_status(&mut self, status: UserStatus) {
        if let Some(sender) = &self.sender {
            if let Err(_) = sender.send(ServerMessage::Status {
                status: status.clone(),
            }) {
                self.view.show_error("Falló cambiar estado");
            }
            self.model.update_local_user_status(status);
        }
    }

    /// Solicita la lista de usuarios
    pub fn request_users(&mut self) {
        if let Some(sender) = &self.sender {
            if let Err(_) = sender.send(ServerMessage::Users) {
                self.view.show_error("Falló pedir usuarios");
            }
        }
    }

    /// Envía un mensaje privado
    ///
    /// - `username`: El usuario al que se quiere enviar el mensaje privado
    /// - `text`: El texto del mensaje
    pub fn send_private_message(&mut self, username: String, text: String) {
        if let Some(sender) = &self.sender {
            if let Err(_) = sender.send(ServerMessage::Text { username, text }) {
                self.view.show_error("Falló enviar mensaje privado");
            }
        }
    }

    /// Envía un mensaje público
    ///
    /// - `text`: El texto del mensaje
    pub fn send_public_message(&mut self, text: String) {
        if let Some(sender) = &self.sender {
            if let Err(_) = sender.send(ServerMessage::PublicText { text }) {
                self.view.show_error("Falló enviar mensaje público");
            }
        }
    }

    /// Crea un cuarto
    ///
    /// - `roomname`: El nombre del cuarto a crear.
    pub fn create_room(&mut self, roomname: String) {
        if let Some(sender) = &self.sender {
            if let Err(_) = sender.send(ServerMessage::NewRoom { roomname }) {
                self.view.show_error("Falló crear cuarto");
            }
        }
    }

    /// Invita a usuarios a un cuarto
    ///
    /// - `roomname`: El cuarto
    /// - `usernames`: Los usuarios invitados
    pub fn invite_users(&mut self, roomname: String, usernames: Vec<String>) {
        if let Some(sender) = &self.sender {
            if let Err(_) = sender.send(ServerMessage::Invite {
                roomname,
                usernames,
            }) {
                self.view.show_error("Falló invitar usuarios");
            }
        }
    }

    /// Unirse a un cuarto
    ///
    /// - `roomname`: El cuarto
    pub fn join_room(&mut self, roomname: String) {
        if let Some(sender) = &self.sender {
            if let Err(_) = sender.send(ServerMessage::JoinRoom { roomname }) {
                self.view.show_error("Falló unirse al cuarto");
            }
        }
    }

    /// Solicitar usuarios de un cuarto
    ///
    /// - `roomname`: El cuarto
    pub fn request_room_users(&mut self, roomname: String) {
        if let Some(sender) = &self.sender {
            if let Err(_) = sender.send(ServerMessage::RoomUsers { roomname }) {
                self.view.show_error("Falló solicitar usuarios del cuarto");
            }
        }
    }

    /// Enviar mensaje a un cuarto
    ///
    /// - `roomname`: El nombre del cuarto
    /// - `text`: El mensaje
    pub fn send_room_message(&mut self, roomname: String, text: String) {
        if let Some(sender) = &self.sender {
            if let Err(_) = sender.send(ServerMessage::RoomText { roomname, text }) {
                self.view.show_error("Falló enviar mensaje al cuarto");
            }
        }
    }

    /// Abandonar un cuarto
    ///
    /// - `roomname`: El cuarto a abandonar
    pub fn leave_room(&mut self, roomname: String) {
        if let Some(sender) = &self.sender {
            if let Err(_) = sender.send(ServerMessage::LeaveRoom { roomname }) {
                self.view.show_error("Falló abandonar cuarto");
            }
        }
    }

    /// Desconectarse del chat
    pub fn disconnect(&mut self) {
        if let Some(sender) = &self.sender {
            if let Err(_) = sender.send(ServerMessage::Disconnect) {
                self.view.show_error("Falló desconectarse");
            }
        }

        // Cerrar conexión
        self.sender = None;
        self.model
            .set_connection_state(ConnectionState::Disconnected);
        self.model.update_local_user_connected(false);
        self.view
            .update_connection_state(&self.model.get_connection_state());
    }

    ///
    /// Manejar respuestas del servidor
    ///

    /// Procesa un mensaje recibido del servidor
    ///
    /// - `msg`: El mensaje recibido
    pub fn handle_server_message(&mut self, msg: ClientMessage) {
        match msg {
            ClientMessage::Response {
                operation,
                result,
                extra,
            } => {
                self.handle_response(operation, result, extra);
            }
            ClientMessage::NewUser { username } => {
                self.handle_new_user(username);
            }
            ClientMessage::NewStatus { username, status } => {
                self.handle_new_status(username, status);
            }
            ClientMessage::UserList { users } => {
                self.handle_user_list(users);
            }
            ClientMessage::TextFrom { username, text } => {
                self.handle_text_from(username, text);
            }
            ClientMessage::PublicTextFrom { username, text } => {
                self.handle_public_text_from(username, text);
            }
            ClientMessage::JoinedRoom { roomname, username } => {
                self.handle_joined_room(roomname, username);
            }
            ClientMessage::Invitation { roomname, username } => {
                self.handle_invitation(roomname, username);
            }
            ClientMessage::RoomUserList { roomname, users } => {
                self.handle_room_user_list(roomname, users);
            }
            ClientMessage::RoomTextFrom {
                roomname,
                username,
                text,
            } => {
                self.handle_room_text_from(roomname, username, text);
            }
            ClientMessage::LeftRoom { roomname, username } => {
                self.handle_left_room(roomname, username);
            }
            ClientMessage::Disconnected { username } => {
                self.handle_disconnected(username);
            }
        }
    }

    ///
    /// Implementa la funcionalidad de cada mensaje
    ///

    /// Maneja una respuesta recibida del servidor
    ///
    /// - `operation`:
    /// - `result`:
    /// - `extra`:
    fn handle_response(
        &mut self,
        operation: Operation,
        result: OperationResult,
        extra: Option<String>,
    ) {
        match (&operation, &result) {
            (Operation::Identify, OperationResult::Success) => {
                if let Some(username) = extra {
                    self.model.set_connection_state(ConnectionState::Identified);
                    self.view
                        .update_connection_state(&self.model.get_connection_state());
                    self.view
                        .show_notification(&format!("Identificado como {}", username));
                    self.model.set_local_username(&username);
                }
            }
            (Operation::Identify, OperationResult::UserAlreadyExists) => {
                self.view.show_error("El nombre de usuario ya existe.");
                self.disconnect()
            }
            (Operation::NewRoom, OperationResult::Success) => {
                if let Some(roomname) = extra {
                    self.model.add_room(roomname.clone(), true, false);
                    self.view.add_room(&self.model.get_rooms()[&roomname]);
                    self.view
                        .show_notification(&format!("Cuarto '{}' creado", roomname));
                }
            }
            (Operation::JoinRoom, OperationResult::Success) => {
                if let Some(roomname) = extra {
                    self.model.join_room(&roomname);
                    self.view
                        .show_notification(&format!("Te uniste al cuarto '{}'", roomname));
                }
            }
            (_, OperationResult::Success) => {
                self.view
                    .show_notification(&format!("Operación {:?} exitosa", operation));
            }
            _ => {
                self.view.show_error(&format!(
                    "Error en operación {:?}: {:?} {:?}",
                    operation, result, extra
                ));
            }
        }
    }

    /// Maneja el evento de tener un nuevo usuario
    ///
    /// - `username`: El nombre del nuevo usuario
    fn handle_new_user(&mut self, username: String) {
        self.model
            .add_remote_user(username.clone(), UserStatus::Active);
        self.view
            .add_user(&self.model.get_remote_users()[&username]);
    }

    /// Maneja el evento de que un usuario tiene un nuevo status
    ///
    /// - `username`: El nombre del usuario
    /// - `status`: El nuevo estatus
    fn handle_new_status(&mut self, username: String, status: UserStatus) {
        self.model.update_user_status(&username, status);
        if let Some(user) = self.model.get_remote_users().get(&username) {
            self.view.update_user(&username, user);
        }
    }

    /// Maneja el evento de pedir los usuarios conectados.
    ///
    /// - `users´: Los usuarios conectados actualmente
    fn handle_user_list(&mut self, users: HashMap<String, UserStatus>) {
        self.model.clean_remote_users();
        for (username, status) in users {
            self.model.add_remote_user(username.clone(), status);
        }
        self.view.update_user_list(&self.model.get_remote_users());
    }

    /// Maneja el evento de recibir un mensaje privado
    ///
    /// - `username`: El usuario que envió el mensaje
    /// - `text`: El texto del mensaje
    fn handle_text_from(&mut self, username: String, text: String) {
        let message = ChatMessage::Private {
            from: username.clone(),
            text,
        };
        self.model.add_message(message.clone());
        self.view.show_message(&message);
    }

    /// Maneja el evento de recibir un mensaje público
    ///
    /// - `username`: El usuario que envió el mensaje
    /// - `text`: El texto del mensaje
    fn handle_public_text_from(&mut self, username: String, text: String) {
        let message = ChatMessage::Public {
            from: username,
            text,
        };
        self.model.add_message(message.clone());
        self.view.show_message(&message);
    }

    /// Maneja el evento de que un usuario se une a un cuarto
    ///
    /// - `roomname`: El nombre del cuarto
    /// - `username`: El nombre del usuario que se une
    fn handle_joined_room(&mut self, roomname: String, username: String) {
        if username == self.model.get_local_user().get_username() {
            self.model.join_room(&roomname);
        } else {
            self.model
                .add_room_member(&roomname, username.clone(), UserStatus::Active);
        }

        if let Some(room) = self.model.get_rooms().get(&roomname) {
            self.view.update_room(&roomname, room);
        }
    }

    /// Maneja el evento de pedir la lista de usuarios de un cuarto
    ///
    /// - `roomname`:
    /// - `users`:
    fn handle_room_user_list(&mut self, roomname: String, users: HashMap<String, UserStatus>) {
        if let Some(rooms) = self.model.get_rooms_mut() {
            if let Some(room) = rooms.get_mut(&roomname) {
                room.set_users(users);
                self.view.show_room_members(&roomname, &room.get_users());
            }
        }
    }

    /// Maneja el evento de recibir un mensaje en un cuarto
    ///
    /// - `roomname`:
    /// - `username`:
    /// - `text`:
    fn handle_room_text_from(&mut self, roomname: String, username: String, text: String) {
        let message = ChatMessage::Room {
            roomname: roomname.clone(),
            from: username,
            text,
        };
        self.model.add_message(message.clone());
        self.view.show_message(&message);
    }

    /// Maneja el evento de que un usuario sale de un cuarto
    ///
    /// - `roomname`:
    /// - `username`:
    fn handle_left_room(&mut self, roomname: String, username: String) {
        if username == self.model.get_local_user().get_username() {
            self.model.leave_room(&roomname);
        } else {
            self.model.remove_room_member(&roomname, &username);
        }

        if let Some(room) = self.model.get_rooms().get(&roomname) {
            self.view.update_room(&roomname, room);
        }
    }

    /// Maneja el evento de desconexión del chat de un usuario
    ///
    /// - `username`:
    fn handle_disconnected(&mut self, username: String) {
        self.model.remove_remote_user(&username);
        self.view.remove_user(&username);

        if let Some(rooms) = self.model.get_rooms_mut() {
            for room in rooms.values_mut() {
                room.remove_user(&username);
            }
        }
    }

    /// Maneja el evento de recibir una invitación a un cuarto
    ///
    /// - `roomname`:
    /// - `username`:
    fn handle_invitation(&mut self, roomname: String, username: String) {
        self.model.add_room(roomname.clone(), false, true);
        self.view
            .show_notification(&format!("{} te invitó al cuarto '{}'.", username, roomname,));
    }

    /// Pide ingresar el nombre de usuario para entrar al chat
    pub fn ask_for_username(&mut self) -> String {
        self.view.ask_for_username()
    }

    /// Pide la información del servidor
    pub fn ask_for_server_info(&mut self) -> (String, u16) {
        let (addr, port) = self.view.ask_for_server_info();

        match port.parse() {
            Ok(port) => {
                self.view
                    .show_notification(&format!("Conectando a {}:{}", addr, port));
                (addr, port)
            }
            Err(_) => {
                self.view.show_error("Servidor inválido, intenta de nuevo.");
                self.ask_for_server_info()
            }
        }
    }

    /// Establece la conexión con el servidor
    pub fn connect(&mut self) -> Result<(), ClientError> {
        let (addr, port) = self.ask_for_server_info();

        let (reader, writer) = connect(&addr, port)?;

        self.reader = Some(reader);
        self.writer = Some(writer);

        self.model.update_local_user_connected(true);
        self.model.set_connection_state(ConnectionState::Connected);
        self.view
            .update_connection_state(&self.model.get_connection_state());
        Ok(())
    }
}

/// Maneja la recepción de un mensaje del servidor
///
/// - `controller`:
pub fn receive_messages_loop(
    controller: Arc<Mutex<ChatController<ConsoleView>>>,
    mut reader: NetworkReader,
) {
    loop {
        match reader.receive_message() {
            Ok(msg) => {
                if let Ok(mut ctrl) = controller.lock() {
                    ctrl.handle_server_message(msg);
                }
            }
            Err(_) => {
                eprintln!("Conexión con el servidor perdida.");
                break;
            }
        }
    }
}

/// Maneja el envío de mensajes al servidor usando canales
///
/// - `mut writer`: Writer de red separado
/// - `receiver`: Canal para recibir mensajes a enviar
pub fn send_messages_loop(mut writer: NetworkWriter, receiver: mpsc::Receiver<ServerMessage>) {
    while let Ok(msg) = receiver.recv() {
        if writer.send_message(&msg).is_err() {
            eprintln!("Error enviando mensaje al servidor");
            break;
        }
    }
}

/// Maneja la interacción con el usuario
///
/// - `controller`:
pub fn user_interaction_loop(controller: Arc<Mutex<ChatController<ConsoleView>>>) {
    loop {
        let is_identified = {
            match controller.lock() {
                Ok(ctrl) => ctrl.is_identified(),
                Err(_) => false,
            }
        };

        if is_identified {
            break;
        }

        // Esperar un poco antes de verificar de nuevo
        std::thread::sleep(std::time::Duration::from_millis(500));
    }

    println!("\n=== Chat Client ===");
    println!("Comandos disponibles:");
    println!("  <texto>          - Mensaje público\n");
    println!("  /users           - Lista de usuarios");
    println!("  /status <status>  - Cambiar estado (ACTIVE|AWAY|BUSY)");
    println!("  /pm <user> <msg>  - Mensaje privado");
    println!("  /rooms           - Lista de cuartos");
    println!("  /create <room>   - Crear cuarto");
    println!("  /join <room>     - Unirse a cuarto");
    println!("  /leave <room>    - Abandonar cuarto");
    println!("  /invite <room> <user1>,<user2>,... - Invitar usuarios");
    println!("  /roomusers <room> - Usuarios en cuarto");
    println!("  /room <room> <msg> - Mensaje a cuarto");
    println!("  /quit            - Salir");

    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    loop {
        let _ = io::stdout().flush();

        if let Some(Ok(line)) = lines.next() {
            if line.is_empty() {
                continue;
            }

            if line.starts_with('/') {
                handle_command(&controller, &line)
            } else {
                match controller.lock() {
                    Ok(mut ctrl) => ctrl.send_public_message(line),
                    Err(_) => {}
                }
            }
        } else {
            break;
        }
    }

    match controller.lock() {
        Ok(mut ctrl) => {
            ctrl.disconnect();
        }
        Err(_) => {}
    }
}

fn handle_command(controller: &Arc<Mutex<ChatController<ConsoleView>>>, command: &str) {
    let parts: Vec<&str> = command.split_whitespace().collect();

    match parts.get(0) {
        Some(&"/users") => match controller.lock() {
            Ok(mut ctrl) => {
                ctrl.request_users();
            }
            Err(_) => {}
        },
        Some(&"/status") => {
            if parts.len() >= 2 {
                let status = match parts[1].to_uppercase().as_str() {
                    "ACTIVE" => UserStatus::Active,
                    "AWAY" => UserStatus::Away,
                    "BUSY" => UserStatus::Busy,
                    _ => {
                        println!("Estado inválido. Use: ACTIVE, AWAY, o BUSY");
                        return;
                    }
                };
                controller.lock().unwrap().set_status(status);
            } else {
                println!("Uso: /status <ACTIVE|AWAY|BUSY>");
            }
        }
        Some(&"/pm") => {
            if parts.len() >= 3 {
                let username = parts[1].to_string();
                let text = parts[2..].join(" ");
                controller
                    .lock()
                    .unwrap()
                    .send_private_message(username, text);
            } else {
                println!("Uso: /pm <usuario> <mensaje>");
            }
        }
        Some(&"/rooms") => {
            println!("\n=== Cuartos ===");
            for (roomname, room) in controller.lock().unwrap().model.get_rooms() {
                let status = if room.get_is_joined() {
                    "[UNIDO]"
                } else if room.get_is_invited() {
                    "[INVITADO]"
                } else {
                    ""
                };
                println!(
                    "  {} {} ({} miembros)",
                    roomname,
                    status,
                    room.get_users().len()
                );
            }
        }
        Some(&"/create") => {
            if parts.len() >= 2 {
                let roomname = parts[1].to_string();
                controller.lock().unwrap().create_room(roomname);
            } else {
                println!("Uso: /create <nombre_cuarto>");
            }
        }
        Some(&"/join") => {
            if parts.len() >= 2 {
                let roomname = parts[1].to_string();
                controller.lock().unwrap().join_room(roomname);
            } else {
                println!("Uso: /join <nombre_cuarto>");
            }
        }
        Some(&"/leave") => {
            if parts.len() >= 2 {
                let roomname = parts[1].to_string();
                controller.lock().unwrap().leave_room(roomname);
            } else {
                println!("Uso: /leave <nombre_cuarto>");
            }
        }
        Some(&"/invite") => {
            if parts.len() >= 3 {
                let roomname = parts[1].to_string();
                let usernames: Vec<String> =
                    parts[2].split(',').map(|s| s.trim().to_string()).collect();
                controller.lock().unwrap().invite_users(roomname, usernames);
            } else {
                println!("Uso: /invite <nombre_cuarto> <usuario1>,<usuario2>,...");
            }
        }
        Some(&"/roomusers") => {
            if parts.len() >= 2 {
                let roomname = parts[1].to_string();
                controller.lock().unwrap().request_room_users(roomname);
            } else {
                println!("Uso: /roomusers <nombre_cuarto>");
            }
        }
        Some(&"/room") => {
            if parts.len() >= 3 {
                let roomname = parts[1].to_string();
                let text = parts[2..].join(" ");
                controller.lock().unwrap().send_room_message(roomname, text);
            } else {
                println!("Uso: /room <nombre_cuarto> <mensaje>");
            }
        }
        Some(&"/quit") => {
            controller.lock().unwrap().disconnect();
            std::process::exit(0);
        }
        Some(cmd) => {
            println!("Comando desconocido: {}", cmd);
        }
        None => {}
    }
}
