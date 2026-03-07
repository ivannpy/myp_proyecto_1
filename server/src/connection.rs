use std::sync::mpsc;

#[derive(Clone)]
pub struct Connection {
    channel: mpsc::Sender<String>,
}

impl Connection {
    pub fn new(channel: mpsc::Sender<String>) -> Self {
        Self {
            channel
        }
    }
    pub fn send(&self, msg: String) {
        let _ = self.channel.send(msg);
    }
}