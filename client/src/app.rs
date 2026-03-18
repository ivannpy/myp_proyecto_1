use crate::client_error::AppError;
use crate::controller::chat_controller::ChatController;
use crate::controller::chat_controller::{receive_messages_loop, user_interaction_loop};
use crate::view::console_view::ConsoleView;
use std::sync::{Arc, Mutex};
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
        match self.controller.lock() {
            Ok(mut controller) => {
                controller.init();
            }
            Err(_) => {
                return Err(AppError::Error);
            }
        }

        let network_ctrl = Arc::clone(&self.controller);
        let interaction_ctrl = Arc::clone(&self.controller);

        let _network_handle = thread::spawn(move || receive_messages_loop(network_ctrl));
        let interaction_handle = thread::spawn(move || user_interaction_loop(interaction_ctrl));

        // Esperar a que el hilo de interacción termine
        let _ = interaction_handle.join();

        Ok(())
    }
}
