/// Errores que pueden ocurrir en el cliente
#[derive(Debug)]
pub enum ClientError {
    ConnectionError,
}

/// Errores que pueden ocurrir en la aplicación
pub enum AppError {
    Error,
}
