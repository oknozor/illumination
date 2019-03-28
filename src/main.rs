extern crate gtk;

#[macro_use]
extern crate horrorshow;

use gtk::*;

pub mod ui;
pub mod nvim;
pub mod preview;

use ui::App;

fn main() {
    let app = App::new();
    app.window.show_all();
    app.connect_nvim();
    gtk::main();
}