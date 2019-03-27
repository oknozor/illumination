use std::process;
use gtk;
use gtk::*;
use super::header::Header;
use super::content::Content;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use crate::preview::render;
use neovim_lib::Neovim;
use neovim_lib::Session;
use sourceview::Buffer;
use webkit2gtk::*;

pub struct App {
    pub window: Window,
    pub header: Header,
    pub content: Content,
}

pub struct ConnectedApp(App);

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

/*
        content.preview.load_html(&render(&buffer), None);
*/

        App { window, header, content }
    }

/*    pub fn connect_events(self) -> ConnectedApp<'app> {
        {
            // Connect all of the events that this UI will act upon.
            self.editor_changed(get_current_buffer(self.nvim));
        }

        // Wrap the `App` within `ConnectedApp` to enable the developer to execute the program.
        ConnectedApp(self)
    }*/

/*    fn editor_changed(&self, buffer: String) {
        let preview = self.content.preview.clone();
        let buffer = Buffer::();
        self.content.buffer.connect_changed(move |last_buffer| {
            if buffer != self.content.buffer {
                preview.load_html(&render(&buffer), None);
                self.content.buffer = buffer;
            }
        });
    }*/
}