extern crate gtk;
#[macro_use]
extern crate horrorshow;
#[macro_use]
extern crate log;

use gtk::*;

use ui::App;

pub mod nvim;
pub mod preview;
pub mod ui;

fn main() {
    logger::init().expect("Error initializing logger");
    info!("Illumination started in debud mode");
    let app = App::new();
    app.window.show_all();
    app.connect_nvim();
    gtk::main();
}

// A simple logger only available if compiled with debug attribute
mod logger {
    use log::{Level, LevelFilter, Metadata, Record, SetLoggerError};

    pub struct SimpleLogger;

    pub static LOGGER: SimpleLogger = SimpleLogger;

    impl log::Log for SimpleLogger {
        fn enabled(&self, metadata: &Metadata) -> bool {
            metadata.level() <= Level::Info
        }

        fn log(&self, record: &Record) {
            if self.enabled(record.metadata()) {
                println!("{} - {}", record.level(), record.args());
            }
        }

        fn flush(&self) {}
    }

    pub fn init() -> Result<(), SetLoggerError> {
        log::set_logger(&LOGGER).map(|()| log::set_max_level(LevelFilter::Info))
    }
}
