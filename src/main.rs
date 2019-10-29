extern crate gtk;
#[macro_use]
extern crate horrorshow;
#[macro_use]
extern crate log;
#[macro_use]
extern crate lazy_static;

extern crate config;
extern crate dirs;

mod doc_lang;
mod html;
mod nvim;
mod preview;
mod settings;
mod ui;
mod logger;

use gtk::*;
use ui::App;

fn main() {
    logger::init().expect("Error initializing logger");
    info!("Illumination started in debud mode");
    #[cfg(debug_assertions)]
    settings::show();

    let app = App::new();
    app.window.show_all();
    app.connect_nvim();
    gtk::main();
}


