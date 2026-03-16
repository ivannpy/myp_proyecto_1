
#[derive(Debug, Clone)]
pub enum ConnectionState {
    Connected,
    Connecting,
    Disconnected,
    Disconnecting,
    Identifying,
    Identified,
    Error(String),
}
