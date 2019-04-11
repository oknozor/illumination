use std::sync::{Mutex, Arc};

use neovim_lib::{Neovim, NeovimApi, Session, UiAttachOptions};
use crate::nvim::handler::Message::*;
use webkit2gtk::*;
use crate::preview::render;
use fragile::Fragile;

pub struct NvimHandler {
    nvim: Neovim,
}

enum Message {
    BufferUpdate,
    Unknown(String),
}

impl From<String> for Message {
    fn from(event: String) -> Self {
        match &event[..] {
            "nvim_buf_lines_event" => BufferUpdate,
            _ => Message::Unknown(event),
        }
    }
}

impl NvimHandler {
    pub fn new() -> NvimHandler {

        #[cfg(debug_assertions)]
            let session = Session::new_tcp("127.0.0.1:6666").unwrap();

        #[cfg(not(debug_assertions))]
            let session = Session::new_parent().unwrap();

        let nvim = Neovim::new(session);
        NvimHandler { nvim }
    }

    pub fn revc(&mut self, shared_buffer: Arc<Mutex<Fragile<WebView>>>) {
        let receiver = self.nvim.session.start_event_loop_channel();

        // Attach current buffer event to the channel
        let mut current_buffer = self.nvim.get_current_buf().unwrap();
        current_buffer.attach(&mut self.nvim, true, vec![]).unwrap();

        // Attach to UI just to get redraw notification, so we make sure every options is deactivated
        let mut ui_options = UiAttachOptions::new();
        ui_options.set_tabline_external(false);
        ui_options.set_cmdline_external(false);
        ui_options.set_hlstate_external(false);
        ui_options.set_linegrid_external(false);
        ui_options.set_tabline_external(false);
        ui_options.set_popupmenu_external(false);
        ui_options.set_rgb(false);
        ui_options.set_wildmenu_external(false);
        self.nvim.ui_attach(100, 100, &ui_options).unwrap();

        // Listen for updates
        for (event, _values) in receiver {
            info!("reveived rpc message : {}", event.clone());
            let fragile_webview = shared_buffer.clone();
            let len = current_buffer.line_count(&mut self.nvim).unwrap();
            
            let current_buffer_id = self.nvim
                .get_current_buf()
                .unwrap()
                .get_number(&mut self.nvim)
                .unwrap();

            // Reattach the new buffer on change
            let active_buffer_id = current_buffer.get_number(&mut self.nvim).unwrap();
            if active_buffer_id != current_buffer_id {
                info!("Buffer changed detached buffer [{}], reattaching buffer, [{}]", current_buffer_id, active_buffer_id);
                current_buffer.detach(&mut self.nvim).expect("Unable to detach buffer");
                current_buffer = self.nvim.get_current_buf().unwrap();
                current_buffer.attach(&mut self.nvim, true, vec![]).unwrap();
            };

            // Update on buff_line_event
            match Message::from(event) {
                BufferUpdate => {
                    let str_buffer = current_buffer
                        .get_lines(&mut self.nvim, 0, len, true)
                        .unwrap()
                        .iter()
                        .map(|line| format!("{}\n", line.to_owned()))
                        .collect::<String>();

                    glib::MainContext::default().invoke(move || {
                        let webview_lock = fragile_webview.lock().unwrap();
                        webview_lock.get().load_html(&render(&str_buffer), None);
                        let context = Fragile::new(webview_lock.get().get_javascript_global_context().unwrap());
                        webview_lock.get().run_javascript("document.body.scrollHeight", None, move |msg| {
                            println!("{:?}", msg.unwrap().get_value().unwrap().to_number(&context.get()));
                        });
                    });
                },
                Unknown(_err_event) => {}

            };
        }
    }
}


