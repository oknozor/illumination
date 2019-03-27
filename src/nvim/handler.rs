pub struct NvimWrapper {
    nvim: Neovim,
}

pub struct BufferHandler {}

use neovim_lib::{Neovim, NeovimApi, Session, Handler, Value, RequestHandler};
use std::fmt::Error;


impl NvimWrapper {
    pub fn init() -> NvimWrapper {
        let mut session = Session::new_tcp("127.0.0.1:6666").unwrap();
        session.start_event_loop_handler(BufferHandler {});

        let mut nvim = Neovim::new(session);
        nvim.subscribe("TextChanged").expect("Unable to suscribe to Neovim event");

        NvimWrapper { nvim }
    }

/*    pub fn handle(&mut self) -> String {
        let reveiver = self.nvim.session.start_event_loop_channel();

        let mut modified_buffer = String::new();
        for (event, value) in reveiver {
            let buffers = self.nvim.list_bufs().unwrap();
            let buffer = &buffers[0];

            let len = buffer.line_count(&mut self.nvim).unwrap();
            modified_buffer = buffer
                .get_lines(&mut self.nvim, 0, len, true)
                .expect("Unable to get nvim buffer")
                .iter()
                .map(|line| format!("{}\n", line.to_owned()))
                .collect()
        }

        modified_buffer
    }*/
}

impl Handler for BufferHandler {
    fn handle_notify(&mut self, _name: &str, _args: Vec<Value>) {
        println!("OK");
    }
}

impl RequestHandler for BufferHandler {
    fn handle_request(&mut self, _name: &str, _args: Vec<Value>) -> Result<Value, Value> {
        println!("NOTIFY");
        Err(Value::from("Not implemented"))
    }
}

