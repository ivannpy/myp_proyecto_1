use crate::client_error::AppError;
use crate::controller::chat_controller::ChatController;
use crate::controller::chat_controller::{
    receive_messages_loop, send_messages_loop, user_interaction_loop,
};
use crate::model::connection_state::ConnectionState;
use crate::view::console_view::ConsoleView;
use std::sync::{Arc, Mutex, mpsc};
use std::thread;

/// Aplicación
/// Cliente para la aplicación de chat.
///
/// - `controller`: Controlador de la aplicación que maneja la lógica de chat.
///                 El controlador toma la vista e internamente maneja el modelo.
pub struct App {
    controller: Arc<Mutex<ChatController<ConsoleView>>>,
}

impl App {
    /// Crea una nueva instancia de la aplicación con la vista dada
    ///
    /// - `view`: La vista que se utilizará para mostrar la interfaz de usuario.
    pub fn new(view: ConsoleView) -> Self {
        Self {
            controller: Arc::new(Mutex::new(ChatController::new(view))),
        }
    }

    /// Inicia la aplicación
    pub fn run(&mut self) -> Result<(), AppError> {
        let (reader, writer) = {
            match self.controller.lock() {
                Ok(mut controller) => {
                    loop {
                        match controller.connect() {
                            Ok(_) => break,
                            Err(_) => {}
                        };
                    }

                    let reader = controller.reader.take().ok_or(AppError::Error)?;
                    let writer = controller.writer.take().ok_or(AppError::Error)?;

                    controller.model.update_local_user_connected(true);
                    controller
                        .model
                        .set_connection_state(ConnectionState::Connected);
                    (reader, writer)
                }
                Err(_) => return Err(AppError::Error),
            }
        };

        // Crear canal para envío de mensajes
        let (sender, receiver) = mpsc::channel();

        if let Ok(mut controller) = self.controller.lock() {
            controller.set_sender(sender);
        }

        let network_ctrl = Arc::clone(&self.controller);
        let interaction_ctrl = Arc::clone(&self.controller);

        let _receive_handle = thread::spawn(move || receive_messages_loop(network_ctrl, reader));

        let _send_handle = thread::spawn(move || send_messages_loop(writer, receiver));

        match self.controller.lock() {
            Ok(mut controller) => match controller.identify() {
                Ok(_) => {}
                Err(_) => {}
            },
            Err(_) => {}
        }

        let interaction_handle = thread::spawn(move || user_interaction_loop(interaction_ctrl));
        let _ = interaction_handle.join();

        Ok(())
    }
}
