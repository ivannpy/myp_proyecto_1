mod app;
mod client_error;
mod controller;
mod model;
mod network_connection;
mod view;

use crate::app::App;
use crate::view::console_view::ConsoleView;

/// Punto de entrada de la aplicación del lado del cliente.
fn main() {
    let mut app = App::new(ConsoleView::new());
    if app.run().is_err() {
        eprintln!("Error al ejecutar la aplicación")
    }
}
