use std::sync::{Arc, Mutex};

use crate::html::theme::Theme;
use crate::nvim::handler::Message::*;
use crate::settings::THEME;
use fragile::Fragile;
use neovim_lib::{Neovim, NeovimApi, Session, UiAttachOptions};
use webkit2gtk::WebView;

type SharedWebView = Arc<Mutex<Fragile<WebView>>>;
type SharedF64 = Arc<Mutex<f64>>;

pub enum GtkMessage {
    Redraw(String),
}

pub struct NvimHandler {
    nvim: Neovim,
    sender: glib::Sender<GtkMessage>,
    current_theme: Theme,
    scroll_value: SharedF64,
}

enum Message {
    Redraw,
    BufferUpdate,
    Unknown(String),
}

impl From<String> for Message {
    fn from(event: String) -> Self {
        match &event[..] {
            "redraw" => Redraw,
            "nvim_buf_lines_event" => BufferUpdate,
            _ => Message::Unknown(event),
        }
    }
}

impl NvimHandler {
    pub fn new(sender: glib::Sender<GtkMessage>) -> NvimHandler {
        #[cfg(debug_assertions)]
        let session = Session::new_tcp("127.0.0.1:6666").unwrap();

        #[cfg(not(debug_assertions))]
        let session = Session::new_parent().unwrap();

        let nvim = Neovim::new(session);
        NvimHandler {
            nvim,
            sender,
            current_theme: THEME.lock().unwrap().theme,
            scroll_value: Arc::new(Mutex::new(f64::from(0))),
        }
    }

    // convert buffer lines to String
    fn curr_buff_to_string(&mut self) -> String {
        let buffer = self.nvim.get_current_buf().unwrap();
        let total_lines = buffer.line_count(&mut self.nvim).unwrap();
        buffer
            .get_lines(&mut self.nvim, 0, total_lines, true)
            .unwrap()
            .iter()
            .map(|line| format!("{}\n", line.to_owned()))
            .collect::<String>()
    }

    pub fn revc(&mut self) {
        // Start the rpc event loop
        let receiver = self.nvim.session.start_event_loop_channel();
        let (g_sender, _) = glib::MainContext::channel::<GtkMessage>(glib::PRIORITY_DEFAULT);

        // Attach current buffer event to the channel
        let mut current_buffer = self.nvim.get_current_buf().unwrap();
        let buffer = self.curr_buff_to_string();

        let _ = g_sender.send(GtkMessage::Redraw(buffer));

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

        let js_window_height = Arc::new(Mutex::new(0.0));

        // Listen for updates
        for (event, _values) in receiver {
            info!("reveived rpc message : {}", event.clone());

            let current_win = self.nvim.get_current_win().unwrap();
            let cursor = current_win.get_cursor(&mut self.nvim);
            let cursor_offset = current_buffer
                .get_offset(&mut self.nvim, cursor.unwrap().0)
                .unwrap_or(0);
            let total_line = current_buffer.line_count(&mut self.nvim).unwrap();
            let total_lenght = current_buffer
                .get_offset(&mut self.nvim, total_line)
                .unwrap();
            let win_height = current_win.get_height(&mut self.nvim);
            let win_width = current_win.get_width(&mut self.nvim);
            let win_position = current_win.get_position(&mut self.nvim);

            info!(
                "window geometry : witdh {:?}, height {:?}",
                win_width, win_height
            );

            info!("window position : {:?}", win_position);
            info!("cursor position : {:?}", cursor_offset);

            let current_buffer_id = self
                .nvim
                .get_current_buf()
                .unwrap()
                .get_number(&mut self.nvim)
                .unwrap();

            // Reattach the new buffer on change
            let active_buffer_id = current_buffer.get_number(&mut self.nvim).unwrap();
            if active_buffer_id != current_buffer_id {
                let new_buffer_name = current_buffer.get_name(&mut self.nvim);
                info!(
                    "Buffer changed detached buffer [{}], reattaching buffer, id=[{}], name= [{}]",
                    current_buffer_id,
                    active_buffer_id,
                    new_buffer_name.unwrap_or("Unknown".into())
                );
                current_buffer
                    .detach(&mut self.nvim)
                    .expect("Unable to detach buffer");
                current_buffer = self.nvim.get_current_buf().unwrap();
                current_buffer.attach(&mut self.nvim, true, vec![]).unwrap();
            };

            // Update on buff_line_event
            match Message::from(event) {
                Redraw => {
                    let js_window_height_inner = Arc::clone(&js_window_height);
                    info!(
                        "cursor offset {}, buffer lenght {}",
                        cursor_offset, total_lenght
                    );

                    let cursor_pos_percent = (cursor_offset as f64 / total_lenght as f64) * 100.0;
                    info!("cursor at {}%", cursor_pos_percent);

                    let js_window_height_inner = js_window_height_inner.lock().unwrap();

                    let scroll_target =
                        (*js_window_height_inner / 100.0) * cursor_pos_percent as f64;
                    info!("webkit inner height {:?}", js_window_height_inner);

                    let js_window_height_inner = Arc::clone(&js_window_height);
                }

                BufferUpdate => {
                    let buffer = self.curr_buff_to_string();
                    let _res = self.sender.send(GtkMessage::Redraw(buffer));
                }

                Unknown(_err_event) => {
                    // We can safely ignore unkown rpc message
                }
            };
        }
    }
}
