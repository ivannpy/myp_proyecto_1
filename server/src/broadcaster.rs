use crate::model::server_state::ServerState;
use protocol::messages::client_message::ClientMessage;
use std::collections::HashMap;
use std::sync::mpsc;

///
/// Para desacoplar a los usuarios del canal que usa su cliente y el servidor para comunicarse.
///
pub struct Broadcaster {
    clients: HashMap<usize, mpsc::Sender<ClientMessage>>,
}

impl Broadcaster {
    pub fn new() -> Self {
        Self {
            clients: HashMap::new(),
        }
    }

    pub fn add_client(&mut self, id: usize, sender: mpsc::Sender<ClientMessage>) {
        self.clients.insert(id, sender);
    }

    pub fn remove_client(&mut self, id: usize) {
        self.clients.remove(&id);
    }

    pub fn send_message_to(&self, id: usize, msg: &ClientMessage) {
        let sender = self.clients.get(&id);
        if let Some(sender) = sender {
            sender.send(msg.clone()).unwrap();
        }
    }

    pub fn send_message_to_user(&self, username: &str, msg: &ClientMessage, state: &ServerState) {
        let user = state.get_users().get(username).unwrap();
        self.send_message_to(user.id, msg);
    }
}
