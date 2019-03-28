pub struct NvimHandler {
    nvim: Neovim,
}

use std::io;
use std::io::{Read, Write};
use std::fs::File;
use std::sync::{Mutex, Arc};


pub struct BufferHandler {}

use neovim_lib::{Neovim, NeovimApi, Session, Handler, Value, RequestHandler};
use std::fmt::Error;
use crate::nvim::handler::Messages::*;
use std::sync::mpsc::Sender;
use neovim_lib::neovim_api::Buffer;
use webkit2gtk::*;
use core::borrow::BorrowMut;
use crate::preview::render;
use fragile::Fragile;
use std::rc::Rc;

enum Messages {
    BufferChanged,
    Unknown(String),
}


impl From<String> for Messages {
    fn from(event: String) -> Self {
        match &event[..] {
            "buffer_changed" => BufferChanged,
            _ => Messages::Unknown(event),
        }
    }
}

impl NvimHandler {
    pub fn new() -> NvimHandler {
        let session = Session::new_tcp("127.0.0.1:6666").unwrap();
        let mut nvim = Neovim::new(session);
        NvimHandler { nvim }
    }

    pub fn revc(&mut self, shared_buffer: Arc<Mutex<Fragile<WebView>>>) {
        let receiver = self.nvim.session.start_event_loop_channel();
        let buffer = self.nvim.get_current_buf().unwrap();
        buffer.attach(&mut self.nvim, true, vec![]);

        for (event, values) in receiver {
            let shared_buffer = shared_buffer.clone();
            let len = buffer.line_count(&mut self.nvim).unwrap();
            println!(" received event : {}", event);
            let str_buffer = buffer.get_lines(&mut self.nvim, 0, len, true)
                                   .unwrap()
                                   .iter()
                                   .map(|line| format!("{}\n", line.to_owned()))
                                   .collect::<String>();

            if event == "nvim_buf_lines_event" {
                glib::MainContext::default().invoke(move || {
                    let buff = shared_buffer.lock().unwrap();
                    buff.get().load_html(&render(&str_buffer), None);
                });
            }
        }
    }
}


