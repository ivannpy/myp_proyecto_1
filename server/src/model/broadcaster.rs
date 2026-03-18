use protocol::messages::client_message::ClientMessage;
use std::collections::HashMap;
use std::sync::mpsc;

/// Mensajeador del servidor para enviar mensajes a los clientes.
/// 
/// - `clients`: Un diccionario con los id de los clientes y su respectivo canal de comunicación.
pub struct Broadcaster {
    clients: HashMap<usize, mpsc::Sender<ClientMessage>>,
}

impl Broadcaster {
    /// Crea una nueva instancia del broadcaster sin clientes registrados.
    pub fn new() -> Self {
        Self {
            clients: HashMap::new(),
        }
    }
    
    /// Registra el canal de comunicación de un cliente en el broadcaster.
    /// 
    /// - `id`: Identificador único del cliente.
    /// - `sender`: Canal de comunicación del cliente con el servidor.
    pub fn add_client(&mut self, id: usize, sender: mpsc::Sender<ClientMessage>) {
        self.clients.insert(id, sender);
    }
    
    /// Elimina el registro del canal de comunicación de un cliente.
    ///
    /// - `id`: Identificador único del cliente que se va a eliminar.
    pub fn remove_client(&mut self, id: usize) {
        self.clients.remove(&id);
    }
    
    /// Envía un mensaje a un cliente.
    /// 
    /// - `id`: El identificador del cliente al que se manda el mensaje.
    /// - `msg`: El mensaje que se va a enviar.
    fn send_message(&self, id: usize, msg: &ClientMessage) {
        if let Some(sender) = self.clients.get(&id) {
            let _ = sender.send(msg.clone());
        }
    }
    
    /// Envía un mensaje a una secuencia de clientes.
    /// 
    /// - `ids`: La secuencia de identificadores de clientes a los que se manda el mensaje.
    /// - `msg`: El mensaje que se va a enviar.
    pub fn send_message_to(&self, ids: Vec<usize>, msg: &ClientMessage) {
        for id in ids {
            let _ = self.send_message(id, msg);
        }
    }
    
    /// Envía un mensaje a todos los clientes excepto al cliente con el id especificado.
    /// 
    /// - `id`: El identificador del cliente que se excluye del envío.
    /// - `msg`: El mensaje que se va a enviar.
    pub fn send_message_to_all_except(&self, id: usize, msg: &ClientMessage) {
        for id_client in self.clients.keys() {
            if *id_client != id {
                let _ = self.send_message(*id_client, msg);
            }
        }
    }
}

#[cfg(test)]
mod tests {}
