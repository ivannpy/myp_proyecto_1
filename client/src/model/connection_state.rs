#[derive(Debug, Clone)]
pub enum ConnectionState {
    Connected,
    Disconnected,
    Identified,
}
