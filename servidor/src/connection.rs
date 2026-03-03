use std::sync::mpsc;

pub struct Connection {
    channel: mpsc::Sender<String>,
}

impl Connection {
    pub fn new(channel: mpsc::Sender<String>) -> Self {
        Self {
            channel,
        }
    }
}