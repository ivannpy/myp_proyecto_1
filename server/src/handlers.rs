use crate::model::{ServerState, User};
use crate::utils::parse_msg_to_json;
use std::collections::HashMap;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::net::TcpStream;
use std::sync::{Arc, Mutex, mpsc};

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
                let msg = line.trim();
                println!("<<< {}", msg);
                let data = parse_msg_to_json(msg);
                handle_action(&state, &data, &sender)
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

/*
   Maneja una accion de un cliente.
*/
fn handle_action(
    state: &Arc<Mutex<ServerState>>,
    data: &HashMap<String, String>,
    sender: &mpsc::Sender<String>,
) {
    let msg_type = data.get("type");
    match msg_type {
        Some(msg_type) => {
            let msg_type = msg_type.as_str();
            match msg_type {
                "IDENTIFY" => {
                    let reply: String;
                    let username = data.get("username").unwrap().clone();

                    {
                        let mut locked = state.lock().unwrap();
                        if locked.users.contains_key(&username) {
                            let mut reply_hashmap = HashMap::new();
                            reply_hashmap.insert("type".to_string(), "RESPONSE".to_string());
                            reply_hashmap.insert("operation".to_string(), "IDENTIFY".to_string());
                            reply_hashmap
                                .insert("result".to_string(), "USER_ALREADY_EXISTS".to_string());
                            reply_hashmap.insert("extra".to_string(), username.clone());

                            reply = serde_json::to_string(&reply_hashmap).unwrap();
                        } else {
                            let user = User {
                                id: String::from(""),
                                sender: sender.clone(),
                            };
                            
                            locked.users.insert(username.clone(), user);
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
                _ => {}
            }
        }
        None => {}
    }
}
