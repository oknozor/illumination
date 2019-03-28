use std::sync::{Mutex, Arc};

use neovim_lib::{Neovim, NeovimApi, Session};
use crate::nvim::handler::Message::*;
use webkit2gtk::*;
use crate::preview::render;
use fragile::Fragile;

pub struct NvimHandler {
    nvim: Neovim,
}

enum Message {
    BufferChanged,
    Unknown(String),
}


impl From<String> for Message {
    fn from(event: String) -> Self {
        match &event[..] {
            "nvim_buf_lines_event" => BufferChanged,
            _ => Message::Unknown(event),
        }
    }
}

impl NvimHandler {
    pub fn new() -> NvimHandler {
        let session = Session::new_parent().unwrap();
        let nvim = Neovim::new(session);
        NvimHandler { nvim }
    }

    pub fn revc(&mut self, shared_buffer: Arc<Mutex<Fragile<WebView>>>) {
        let receiver = self.nvim.session.start_event_loop_channel();
        let current_buffer = self.nvim.get_current_buf().unwrap();
        current_buffer.attach(&mut self.nvim, true, vec![]).unwrap();

        for (event, _values) in receiver {
            let shared_buffer = shared_buffer.clone();
            let len = current_buffer.line_count(&mut self.nvim).unwrap();

            match Message::from(event) {
                BufferChanged => {
                    let str_buffer = current_buffer
                        .get_lines(&mut self.nvim, 0, len, true)
                        .unwrap()
                        .iter()
                        .map(|line| format!("{}\n", line.to_owned()))
                        .collect::<String>();

                    glib::MainContext::default().invoke(move || {
                        let buff = shared_buffer.lock().unwrap();
                        buff.get().load_html(&render(&str_buffer), None);
                    });
                }

                Unknown(_err_event) => {}
            }
        }
    }
}


