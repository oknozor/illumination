use std::sync::{Arc, Mutex};
use std::{process, thread};

use gtk;
use gtk::*;

use super::content::Content;
use super::header::Header;
use crate::nvim::handler::NvimHandler;
use fragile::Fragile;

pub struct App {
    pub window: Window,
    pub header: Header,
    pub content: Content,
}

impl App {
    pub fn new() -> App {
        if gtk::init().is_err() {
            eprintln!("Failed to initialize GTK Application");
            process::exit(1);
        }

        let window = Window::new(WindowType::Toplevel);
        let header = Header::new();
        let content = Content::new();

        window.set_titlebar(Some(&header.container));
        window.set_title("Illumination");
        window.set_role("Illumination");
        window.add(&content.container);

        window.connect_delete_event(move |_, _| {
            main_quit();
            Inhibit(false)
        });

        App {
            window,
            header,
            content,
        }
    }

    pub fn connect_nvim(self) {
        let webkit = Arc::new(Mutex::new(Fragile::new(self.content.preview.clone())));
        let mut nvim_handler = NvimHandler::new(webkit);

        thread::spawn(move || {
            nvim_handler.revc();
        });
    }
}
