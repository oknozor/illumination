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
use crate::doc_lang;
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

    pub fn first_load(&self, input: String) {
        let theme_selector = self.header.theme_selector.clone();

        let webkit = self.content.preview.clone();
        webkit.load_uri(&input);

        theme_selector.connect_changed(move |combo| {
            let selection = combo.get_active_text().unwrap();
            let selection = selection.as_str();
            info!("changing theme to : {}", selection);
            settings::set_theme(Theme::from(selection));
            webkit.load_uri(&input);
        });
    }

    pub fn standalone_mode(&self, uri: &str) {
        self.first_load("file:///home/okno/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/share/doc/rust/html/std/fs/struct.File.html".to_string());
    }

    pub fn connect_nvim(&self) {
        let (sender, receiver) = glib::MainContext::channel::<GtkMessage>(glib::PRIORITY_DEFAULT);
        let mut nvim_handler = NvimHandler::new(sender);

        let cur_buffer = Arc::new(Mutex::new(String::new()));
        let cur_buffer_ref = Arc::clone(&cur_buffer);

        let webkit = self.content.preview.clone();
        let window = self.window.clone();

        // Attach gtk main thread to the nvim handler sender
        // FIXME : scroll value for update and redraw
        let mut previous_message_was_not_update = true;
        receiver.attach(None, move |msg| {
            match msg {
                GtkMessage::Redraw(scroll_target) => {
                    if previous_message_was_not_update {
                        scroll_to(&webkit, scroll_target);
                    }

                    previous_message_was_not_update = true;
                }

                GtkMessage::BufferUpdate(content, scroll) => {
                    *cur_buffer_ref.lock().unwrap() = content.clone();
                    webkit.load_html(&render(&content, scroll), None);
                    previous_message_was_not_update = false;
                }

                GtkMessage::BufferDetached(title, content, scroll) => {
                    *cur_buffer_ref.lock().unwrap() = content.clone();
                    webkit.load_html(&render(&content, scroll), None);
                    window.set_title(title.as_str());
                    previous_message_was_not_update = false;
                }

                GtkMessage::RustDocOpen => {
                    let uri = doc_lang::rustdoc::get_uri().expect("Unable to get doc path");
                    webkit.load_uri(&format!("file://{}", &uri));
                }
            };

            glib::Continue(true)
        });

        let cur_buffer_ref = Arc::clone(&cur_buffer);
        let cur_buffer = &cur_buffer_ref.lock().unwrap();
        self.first_load(cur_buffer.to_string());

        thread::spawn(move || {
            nvim_handler.revc();
        });
    }
}

fn scroll_to(webview: &WebView, to: f64) {
    let js_scroll = &format!(
        "window.scrollTo(0, document.documentElement.scrollHeight / 100 * {});",
        to
    );
    webview.run_javascript(js_scroll, None::<&gio::Cancellable>, move |msg| {
        info!("webkit window scrolling to : {} px", to);
        info!("js result: {:?} ", msg);
    });
}
