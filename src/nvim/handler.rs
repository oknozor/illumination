use crate::nvim::handler::Message::*;
use neovim_lib::{Integer, Neovim, NeovimApi, Session, UiAttachOptions, Value};

pub enum GtkMessage {
    Redraw(String),
    BufferChanged(String, String),
    RustDocOpen,
}

pub struct NvimHandler {
    nvim: Neovim,
    sender: glib::Sender<GtkMessage>,
    lock: bool,
}

// see neovim :help ui-events
enum Message {
    Redraw,
    Flush,
    BufferUpdate,
    RustDocOpen,
    Lock,
    Unknown(String),
}

impl From<String> for Message {
    fn from(event: String) -> Self {
        match &event[..] {
            "redraw" => Redraw,
            "flush" => Flush,
            "nvim_buf_lines_event" => BufferUpdate,
            "rust_doc_open" => RustDocOpen,
            "lock" => Lock,
            _ => Message::Unknown(event),
        }
    }
}

impl NvimHandler {
    pub fn new(sender: glib::Sender<GtkMessage>) -> NvimHandler {
        // Start a headless client (debug)
        #[cfg(debug_assertions)]
        let session = Session::new_tcp("127.0.0.1:6666").unwrap();

        // Spaw a child process (release) 
        #[cfg(not(debug_assertions))]
        let session = Session::new_parent().unwrap();

        let nvim = Neovim::new(session);
        NvimHandler {
            nvim,
            sender,
            lock: false,
        }
    }

    #[cfg(debug_assertions)]
    fn set_debug_proccess_id(&mut self) {
        let info = self.nvim.get_api_info();
        let client_id = info.unwrap();
        let client_id = client_id.get(0);

        self.nvim
            .set_var("nvimMdJobId", client_id.unwrap().to_owned())
            .unwrap();
    }

    // toggle ui lock: won't switch buffer while locked
    fn toggle_lock(&mut self) {
        if self.lock {
            self.lock = false;
        } else {
            self.lock = true;
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

    fn get_curr_buffer_name(&mut self) -> String {
        let buffer = self.nvim.get_current_buf().unwrap();
        buffer.get_name(&mut self.nvim).unwrap()
    }

    pub fn revc(&mut self) {
        // Start the rpc event loop
        let receiver = self.nvim.session.start_event_loop_channel();
        let (g_sender, _) = glib::MainContext::channel::<GtkMessage>(glib::PRIORITY_DEFAULT);

        // If in debug mod we need to manually set the rpc channel id
        #[cfg(debug_assertions)]
        self.set_debug_proccess_id();

        // Attach current buffer event to the channel
        let mut current_buffer = self.nvim.get_current_buf().unwrap();
        let buffer = self.curr_buff_to_string();

        // Send the first draw message
        let _ = g_sender.send(GtkMessage::Redraw(buffer));

        // Attach current buffer to the ui
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
            info!("received rpc message : {}", event.clone());

            let current_win = self.nvim.get_current_win().unwrap();
            let cursor = current_win.get_cursor(&mut self.nvim);
            let cursor_offset = current_buffer
                .get_offset(&mut self.nvim, cursor.unwrap().0)
                .unwrap_or(0);
            let total_line = current_buffer.line_count(&mut self.nvim).unwrap();
            let _total_lenght = current_buffer
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
                    new_buffer_name.unwrap_or_else(|_| "unknown".to_string())
                );
                current_buffer
                    .detach(&mut self.nvim)
                    .expect("Unable to detach buffer");
                current_buffer = self.nvim.get_current_buf().unwrap();
                current_buffer.attach(&mut self.nvim, true, vec![]).unwrap();
            };

            match Message::from(event) {
                Redraw if !self.lock => {
                    info!("Received rpc message : redraw");
                    let buffer = self.curr_buff_to_string();
                    let buffer_name = self.get_curr_buffer_name();
                    let _res = self
                        .sender
                        .send(GtkMessage::BufferChanged(buffer_name, buffer));
                }

                // Update on buff_line_event
                BufferUpdate => {
                    if !self.lock {
                        info!("Received rpc message : nvim_buf_lines_event");
                        let buffer = self.curr_buff_to_string();
                        let _res = self.sender.send(GtkMessage::Redraw(buffer));
                    }
                }

                // FIXME: why is this not sent ?
                Message::Flush => {
                    info!("Received rpc message : flush");
                }

                Message::RustDocOpen => {
                    self.toggle_lock();
                    let _res = self.sender.send(GtkMessage::RustDocOpen);
                }

                Message::Lock => {
                    self.toggle_lock();
                }

                Message::Unknown(u_event) => {
                    // We can safely ignore unkown rpc message
                    info!("Received unknow rpc message : {}", u_event);
                }
                _ => {}
            };
        }
    }
}
