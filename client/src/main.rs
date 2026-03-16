mod app;
mod controller;
mod model;
mod network_connection;
mod view;
mod client_error;

use crate::app::App;
use crate::view::console_view::ConsoleView;

fn main() {
    println!("=== Iniciando app ===");
    let mut app = App::new(ConsoleView::new());
    app.run();
}
