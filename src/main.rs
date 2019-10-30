extern crate gtk;
#[macro_use]
extern crate horrorshow;
#[macro_use]
extern crate log;
#[macro_use]
extern crate lazy_static;
extern crate clap;
use clap::{App, Arg};

extern crate config;
extern crate dirs;

mod doc_lang;
mod html;
mod logger;
mod nvim;
mod preview;
mod settings;
mod ui;

use gtk::*;
use ui::App as GtkApp;

fn main() {
    logger::init().expect("Error initializing logger");
    info!("Illumination started in debud mode");

    #[cfg(debug_assertions)]
    settings::show();

    let version = env!("CARGO_PKG_VERSION");
    let author = env!("CARGO_PKG_AUTHORS");

    let matches = App::new("Illumination cli")
        .version(version)
        .author(author)
        .about("Does awesome things")
        .arg(
            Arg::with_name("standalone")
                .long("standalone")
                .short("s")
                .value_name("INPUT")
                .takes_value(true)
                .help("Open a single html/mardown file in headless mode"),
        )
        .get_matches();

    println!("starting illumination");

    if matches.is_present("standalone") {
        let app = GtkApp::new();
        let path = matches.value_of("standalone").unwrap();
        println!("Using input file: {}", path);
        app.window.show_all();
        app.standalone_mode(path)
    } else {
        let app = GtkApp::new();
        app.window.show_all();
        app.connect_nvim();
    }

    gtk::main();
}
