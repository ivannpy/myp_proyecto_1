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

    ///
    /// Registra el canal de comunicación de un cliente en el canal broadcaster.
    ///
    /// Un cliente se identifica por su id único y su canal de comunicación con el
    /// servidor es el sender.
    ///
    pub fn add_client(&mut self, id: usize, sender: mpsc::Sender<ClientMessage>) {
        self.clients.insert(id, sender);
    }

    ///
    /// Elimina el registro del canal de comunicación de un cliente.
    ///
    pub fn remove_client(&mut self, id: usize) {
        self.clients.remove(&id);
    }

    ///
    /// Dado el id del cliente, envía un mensaje ClientMessage al cliente vía
    /// el sender registrado para ese cliente.
    ///
    pub fn send_message_to(&self, id: usize, msg: &ClientMessage) -> Result<(), std::io::Error> {
        let sender = self.clients.get(&id);
        match sender {
            Some(sender) => {
                let r = sender.send(msg.clone());
                if r.is_err() {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        "Error al enviar mensaje",
                    ));
                }
                Ok(())
            }
            None => Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "Cliente no registrado",
            )),
        }
    }

    ///
    /// Dado el nombre de usuario del cliente, envía un mensaje ClienteMessage a ese cliente.
    ///
    /// Enviar un mensaje a un usuario puede efectuarse con dos resultados
    /// Ok - Se encontró el usuario y se envió el mensaje
    /// Err - No se encontró el usuario o no se pudo enviar el mensaje
    ///
    pub fn send_message_to_user(
        &self,
        username: &str,
        msg: &ClientMessage,
        state: &ServerState,
    ) -> Result<(), std::io::Error> {
        let user = state.get_users().get(username);
        match user {
            Some(user) => {
                let r = self.send_message_to(user.id, msg);
                if r.is_err() {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        "Error al enviar mensaje",
                    ));
                }
            }
            None => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::NotFound,
                    "Usuario no encontrado",
                ));
            }
        }
        Ok(())
    }
}

mod tests {}
