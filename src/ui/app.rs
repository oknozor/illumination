use std::{process, thread};
use std::sync::{Arc, mpsc, Mutex, RwLock};
use std::sync::atomic::AtomicBool;
use std::sync::mpsc::channel;

use gtk;
use gtk::*;
use neovim_lib::Neovim;
use neovim_lib::Session;
use sourceview::Buffer;
use webkit2gtk::*;

use crate::preview::render;

use super::content::Content;
use super::header::Header;
use crate::nvim::handler::NvimHandler;
use std::fs::File;
use std::io::Write;
use fragile::Fragile;
use glib::spaced_primes_closest;
use pango::lookup_aliases;
use std::rc::Rc;

pub struct App {
    pub window: Window,
    pub header: Header,
    pub content: Content,
}

pub struct ConnectedApp(App);

impl ConnectedApp {
    /// Display the window, and execute the gtk main event loop.
    pub fn then_execute(self) {
        self.0.window.show_all();
        gtk::main();
    }
}

impl<'app> App {
    pub fn new() -> App {
        if gtk::init().is_err() {
            eprintln!("failed to initialize GTK Application");
            process::exit(1);
        }

        let window = Window::new(WindowType::Toplevel);
        let header = Header::new();
        let content = Content::new();


        window.set_titlebar(&header.container);
        window.set_title("NvimRender");
        window.set_wmclass("nvim-render", "NvimRender");
        Window::set_default_icon_name("iconname");
        window.add(&content.container);

        window.connect_delete_event(move |_, _| {
            main_quit();
            Inhibit(false)
        });

        App { window, header, content }
    }

    pub fn connect_events(self) {
        let preview = Arc::new(Mutex::new(Fragile::new(self.content.preview.clone())));
        thread::spawn(move || {
            let mut nvim_handler = NvimHandler::new();
            nvim_handler.revc(Arc::clone(&preview));
        });

    }

}