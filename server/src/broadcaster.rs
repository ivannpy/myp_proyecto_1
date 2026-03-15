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

    ///
    /// Registra el canal de comunicación de un cliente en el canal broadcaster.
    ///
    /// Un cliente se identifica por su id único y su canal de comunicación con el
    /// servidor es el sender.
    ///
    pub fn add_client(&mut self, id: &usize, sender: mpsc::Sender<ClientMessage>) {
        self.clients.insert(id.clone(), sender);
    }

    ///
    /// Elimina el registro del canal de comunicación de un cliente.
    ///
    pub fn remove_client(&mut self, id: usize) {
        self.clients.remove(&id);
    }

    pub fn send_message_to(&self, id: &usize, msg: &ClientMessage) -> Result<(), std::io::Error> {
        let sender = self.clients.get(id);
        if let Some(sender) = sender {
            let _ = sender.send(msg.clone());
            Ok(())
        } else {
            Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "Cliente no registrado",
            ))
        }
    }

    pub fn send_message_to_all_except(&self, id: &usize, msg: &ClientMessage) {
        for (id_client, sender) in self.clients.iter() {
            if id_client != id {
                let r = sender.send(msg.clone());
                if r.is_err() {
                    println!("Error al enviar mensaje a cliente {}", id_client);
                }
            }
        }
    }

    pub fn send_message_to_room(&self, ids: Vec<usize>, msg: &ClientMessage) {
        for id in ids {
            let _ = self.send_message_to(&id, msg);
        }
    }
}

#[cfg(test)]
mod tests {}
