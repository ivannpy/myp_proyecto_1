use crate::controller::chat_controller::ChatController;
use crate::network_connection::NetworkReader;
use crate::view::console_view::ConsoleView;
use protocol::status::user::UserStatus;
use std::io::{BufRead, Write};
use std::sync::{Arc, Mutex};
use std::{io, thread};
use crate::client_error::ClientError;

pub struct App {
    controller: Arc<Mutex<ChatController<ConsoleView>>>,
}

impl App {
    pub fn new(view: ConsoleView) -> Self {
        Self {
            controller: Arc::new(Mutex::new(ChatController::new(view))),
        }
    }

    /// Conecta y devuelve el reader para usarlo después.
    fn connect(&mut self, addr: &str, port: u16) -> Result<NetworkReader, ClientError> {
        match self.controller.lock() {
            Ok(mut ctrl) => ctrl.connect(addr, port),
            Err(_) => Err(ClientError::ConnectionError),
        }
    }

    fn get_username(&self) -> String {
        let stdin = io::stdin();
        let mut lines = stdin.lock().lines();

        println!("Identificate: ");

        loop {
            print!("> ");
            let _ = io::stdout().flush();

            if let Some(Ok(line)) = lines.next() {
                if !line.is_empty() {
                    return line;
                }
            }
        }
    }

    /// Identifica al usuario, usando el reader temporalmente.
    fn identify(&mut self, reader: &mut NetworkReader) -> Result<(), ClientError> {
        let username = self.get_username();
        println!("Username capturado {}", username);

        match self.controller.lock() {
            Ok(mut ctrl) => {
                match ctrl.identify(username, reader) {
                    Ok(_) => {
                        println!("Identificación correcta");
                        Ok(())
                    }
                    Err(_) => Err(ClientError::InvalidUsername),
                }
            }
            Err(_) => Err(ClientError::ConnectionError),
        }
    }

    fn get_server_addr(&self) -> String {
        let stdin = io::stdin();
        let mut lines = stdin.lock().lines();
        println!("Server address: ");
        loop {
            print!("> ");
            let _ = io::stdout().flush();
            if let Some(Ok(line)) = lines.next() {
                if !line.is_empty() {
                    return line;
                }
            }
        }
    }

    fn get_server_port(&self) -> u16 {
        let stdin = io::stdin();
        let mut lines = stdin.lock().lines();
        println!("Server port: ");
        loop {
            print!("> ");
            let _ = io::stdout().flush();
            if let Some(Ok(line)) = lines.next() {
                if let Ok(port) = line.parse::<u16>() {
                    return port;
                }
                println!("Puerto inválido, intenta de nuevo.");
            }
        }
    }

    pub fn run(&mut self) {
        let server_addr = self.get_server_addr();
        let port = self.get_server_port();

        println!("Conectando al servidor en {}:{}", server_addr, port);

        // connect() devuelve el reader
        let mut reader = match self.connect(&server_addr, port) {
            Ok(reader) => reader,
            Err(_) => {
                println!("Error al conectar");
                return;
            }
        };

        // identify() usa el reader temporalmente para la respuesta sincrónica
        match self.identify(&mut reader) {
            Ok(_) => {}
            Err(_) => {
                println!("Error al identificarse");
                return;
            }
        }
        println!("Identificación terminada");

        // El reader se mueve al hilo de recepción (sin lock)
        let network_ctrl = Arc::clone(&self.controller);
        let interaction_ctrl = Arc::clone(&self.controller);

        let _network_handle = thread::spawn(move || receive_messages_loop(reader, network_ctrl));
        let interaction_handle = thread::spawn(move || user_interaction_loop(interaction_ctrl));

        // Esperar a que el hilo de interacción termine
        let _ = interaction_handle.join();
    }
}

/// Loop para recibir mensajes del servidor.
/// El reader vive aquí — NO necesita el lock para leer del socket.
fn receive_messages_loop(
    mut reader: NetworkReader,
    controller: Arc<Mutex<ChatController<ConsoleView>>>,
) {
    loop {
        // Leer del socket SIN tener el lock
        match reader.receive_message() {
            Ok(msg) => {
                // Solo adquirir el lock brevemente para procesar
                if let Ok(mut ctrl) = controller.lock() {
                    ctrl.handle_server_message(msg);
                }
            }
            Err(_) => {
                println!("Conexión con el servidor perdida.");
                break;
            }
        }
    }
}

// Loop para interacción con el usuario (CLI)
fn user_interaction_loop(controller: Arc<Mutex<ChatController<ConsoleView>>>) {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    println!("\n=== Chat Client ===");
    println!("Comandos disponibles:");
    println!("  /users           - Lista de usuarios");
    println!("  /status <status>  - Cambiar estado (ACTIVE|AWAY|BUSY)");
    println!("  /pm <user> <msg>  - Mensaje privado");
    println!("  /rooms           - Lista de cuartos");
    println!("  /create <room>   - Crear cuarto");
    println!("  /join <room>     - Unirse a cuarto");
    println!("  /leave <room>    - Abandonar cuarto");
    println!("  /invite <room> <user1>,<user2>,... - Invitar usuarios");
    println!("  /roomusers <room> - Usuarios en cuarto");
    println!("  /quit            - Salir");
    println!("  <texto>          - Mensaje público\n");

    loop {
        print!("> ");
        let _ = io::stdout().flush();

        if let Some(Ok(line)) = lines.next() {
            if line.is_empty() {
                continue;
            }

            if line.starts_with('/') {
                handle_command(&controller, &line)
            } else {
                controller.lock().unwrap().send_public_message(line);
            }
        } else {
            break;
        }
    }

    // Desconectar antes de salir
    controller.lock().unwrap().disconnect();
}

fn handle_command(controller: &Arc<Mutex<ChatController<ConsoleView>>>, command: &str) {
    let parts: Vec<&str> = command.split_whitespace().collect();

    match parts.get(0) {
        Some(&"/users") => {
            controller.lock().unwrap().request_users();
        }
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
