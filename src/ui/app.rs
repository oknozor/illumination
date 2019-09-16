use crate::html::theme::Theme;
use crate::nvim::handler::GtkMessage;
use crate::preview::render;
use crate::settings;
use std::sync::{Arc, Mutex};
use std::{process, thread};
use webkit2gtk::*;

use gtk::*;

use super::content::Content;

use super::header::Header;
use crate::nvim::handler::NvimHandler;

pub struct App {
    pub window: Window,
    pub header: Header,
    pub content: Content,
    pub buffer: String,
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
            buffer: String::from(""),
        }
    }

    pub fn connect_nvim(&self) {
        let (sender, receiver) = glib::MainContext::channel::<GtkMessage>(glib::PRIORITY_DEFAULT);
        let mut nvim_handler = NvimHandler::new(sender);

        let cur_buffer = Arc::new(Mutex::new(String::new()));
        let cur_buffer_ref = Arc::clone(&cur_buffer);

        let webkit = self.content.preview.clone();
        receiver.attach(None, move |msg| {
            match msg {
                GtkMessage::Redraw(buffer) => {
                    *cur_buffer_ref.lock().unwrap() = buffer.clone();
                    webkit.load_html(&render(&buffer, 0), None);
                }
            };

            glib::Continue(true)
        });

        let combo = self.header.theme_selector.clone();
        let cur_buffer_ref = Arc::clone(&cur_buffer);
        let webkit = self.content.preview.clone();

        combo.connect_changed(move |combo| {
            let selection = combo.get_active_text().unwrap();
            let selection = selection.as_str();
            info!("changing theme to : {}", selection);
            settings::set_theme(Theme::from(selection));
            webkit.load_html(&render(&cur_buffer_ref.lock().unwrap(), 0), None);
        });

        thread::spawn(move || {
            nvim_handler.revc();
        });
    }
}
