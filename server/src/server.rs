use crate::model::ServerState;
use crate::utils::parse_msg_to_json;
use std::collections::HashMap;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::sync::{Arc, Mutex, mpsc};
use std::thread;

pub struct Server {
    pub listener: TcpListener,
    pub state: Arc<Mutex<ServerState>>,
}

impl Server {
    pub fn new(port: u16) -> Result<Self, std::io::Error> {
        let socket_address = SocketAddr::from(([0, 0, 0, 0], port));
        let listener = TcpListener::bind(socket_address)?;

        let state = Arc::new(Mutex::new(ServerState {
            connections: HashMap::new(),
        }));

        Ok(Self { listener, state })
    }

    /*
       Levanta el servidor TCP con sockets.
    */
    pub fn run(&self) -> Result<(), std::io::Error> {
        println!(
            "Servidor {} escuchando en el puerto {}",
            self.listener.local_addr()?.ip(),
            self.listener.local_addr()?.port()
        );

        loop {
            match self.listener.accept() {
                Ok((socket, _)) => {
                    let socket_clone = socket.try_clone();
                    match socket_clone {
                        Ok(socket_clone) => {
                            let reader = BufReader::new(socket_clone);
                            let writer = BufWriter::new(socket);
                            let (sender, receiver) = mpsc::channel::<String>();
                            let state = Arc::clone(&self.state);

                            // TODO: incorporar a bitácora
                            println!("Conexion aceptada");
                            println!("\tCliente: {:?}", reader.get_ref().peer_addr());
                            println!("\tPuerto: {:?}", reader.get_ref().peer_addr()?.port());

                            // Manejar mensajes desde el cliente
                            thread::spawn(|| Self::handle_input_from_client(reader, sender, state));

                            // Manejar mensajes hacia el cliente
                            thread::spawn(|| Self::handle_output_to_client(writer, receiver));
                        }
                        Err(e) => {
                            eprintln!("Error clonando socket: {}", e);
                            continue;
                        }
                    }
                }
                _ => {
                    eprintln!("Error al aceptar conexión");
                    continue;
                }
            }
        }
    }

    fn handle_input_from_client(
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
                    Self::handle_action(&state, &data, &sender)
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

    fn handle_output_to_client(mut writer: BufWriter<TcpStream>, receiver: mpsc::Receiver<String>) {
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
                            if locked.connections.contains_key(&username) {
                                let mut reply_hashmap = HashMap::new();
                                reply_hashmap.insert("type".to_string(), "RESPONSE".to_string());
                                reply_hashmap
                                    .insert("operation".to_string(), "IDENTIFY".to_string());
                                reply_hashmap.insert(
                                    "result".to_string(),
                                    "USER_ALREADY_EXISTS".to_string(),
                                );
                                reply_hashmap.insert("extra".to_string(), username.clone());

                                reply = serde_json::to_string(&reply_hashmap).unwrap();
                            } else {
                                locked.connections.insert(username.clone(), sender.clone());
                                let mut reply_hashmap = HashMap::new();
                                reply_hashmap.insert("type".to_string(), "RESPONSE".to_string());
                                reply_hashmap
                                    .insert("operation".to_string(), "IDENTIFY".to_string());
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
}
