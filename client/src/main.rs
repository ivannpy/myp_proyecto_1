mod app;
mod client_error;
mod controller;
mod model;
mod network_connection;
mod view;

use crate::app::App;
use crate::view::console_view::ConsoleView;

fn main() {
    println!("=== Iniciando app ===");
    let mut app = App::new(ConsoleView::new());
    app.run();
}
