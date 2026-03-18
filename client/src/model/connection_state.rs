/// Estado de conexión del usuario en la aplicación
#[derive(Debug, Clone)]
pub enum ConnectionState {
    Connected,
    Disconnected,
    Identified,
}
