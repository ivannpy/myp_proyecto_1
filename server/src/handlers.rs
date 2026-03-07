use crate::model::{ServerState, User};
use crate::utils::parse_msg_to_json;
use protocol::messages::server_message::ServerMessage;
use std::collections::HashMap;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::net::TcpStream;
use std::sync::{Arc, Mutex, mpsc};

/*
   Maneja la entrada de mensajes desde el cliente.
*/
pub fn handle_input_from_client(
    mut reader: BufReader<TcpStream>,
    sender: mpsc::Sender<String>,
    state: Arc<Mutex<ServerState>>,
) {
    let mut line = String::new();
    loop {
        line.clear();

        match reader.read_line(&mut line) {
            Ok(0) => break,
            Ok(_) => {
                // Manejar mensaje recibido
                let msg_str = line.trim();
                // Parsear linea a ServerMessage
                let data = parse_msg_to_json(msg_str);
                let msg = ServerMessage::Identify {
                    username: data.get("username").unwrap().clone(),
                };
                handle_message(&state, &sender, msg);
                println!("<<< {}", msg_str);
            }
            Err(e) => {
                eprintln!(
                    "Error leyendo de {:?}: {}",
                    reader.get_ref().peer_addr().unwrap(),
                    e
                );
            }
        }
    }
}

/*
   Maneja la salida de mensajes al cliente.
*/
pub fn handle_output_to_client(mut writer: BufWriter<TcpStream>, receiver: mpsc::Receiver<String>) {
    while let Ok(mut msg) = receiver.recv() {
        msg.push('\n');
        println!(">>> {}", msg);
        if writer.write_all(msg.as_bytes()).is_err() {
            break;
        }
        if writer.flush().is_err() {
            break;
        }
    }
}

// Manejadores del protocolo

/*
   Maneja la identificación de un usuario.
*/
fn handle_identify(
    state: &Arc<Mutex<ServerState>>,
    sender: &mpsc::Sender<String>,
    username: String,
) {
    let reply: String;
    {
        let mut locked_state = state.lock().unwrap();
        if locked_state.get_users().contains_key(&username) {
            let mut reply_hashmap = HashMap::new();
            reply_hashmap.insert("type".to_string(), "RESPONSE".to_string());
            reply_hashmap.insert("operation".to_string(), "IDENTIFY".to_string());
            reply_hashmap.insert("result".to_string(), "USER_ALREADY_EXISTS".to_string());
            reply_hashmap.insert("extra".to_string(), username.clone());
            reply = serde_json::to_string(&reply_hashmap).unwrap();
        } else {
            let user = User {
                id: locked_state.get_next_id(),
                sender: sender.clone(),
                username: username.clone(),
            };

            println!(
                "User {} inserted with id {} and sender {:?}",
                username, user.id, user.sender
            );

            locked_state.insert_user(user);
            let mut reply_hashmap = HashMap::new();
            reply_hashmap.insert("type".to_string(), "RESPONSE".to_string());
            reply_hashmap.insert("operation".to_string(), "IDENTIFY".to_string());
            reply_hashmap.insert("result".to_string(), "SUCCESS".to_string());
            reply_hashmap.insert("extra".to_string(), username.clone());
            reply = serde_json::to_string(&reply_hashmap).unwrap();
        }
    }
    sender.send(reply).unwrap();
}

/*
   Manejador de mensajes del protocolo.
*/
pub fn handle_message(
    state: &Arc<Mutex<ServerState>>,
    sender: &mpsc::Sender<String>,
    msg: ServerMessage,
) {
    match msg {
        ServerMessage::Identify { username } => handle_identify(state, sender, username),
    }
}
