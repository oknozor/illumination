extern crate gtk;

#[macro_use]
extern crate horrorshow;

use std::process;

use gtk::*;
use webkit2gtk::WebContext;
use webkit2gtk::WebView;


pub mod ui;
pub mod nvim;
pub  mod preview;

use ui::App;
use crate::nvim::handler::{NvimWrapper, BufferHandler};
use neovim_lib::{Session, Neovim};
use neovim_lib::neovim_api::NeovimApi;

fn main() {

/*
    let context = WebContext::get_default().unwrap();
    let webview = WebView::new_with_context(&context);
    webview.load_html(&html, None);
*/

    let mut session = Session::new_parent().unwrap();
    session.start_event_loop_handler(BufferHandler {});

    let mut nvim = Neovim::new(session);
    nvim.subscribe("changed").expect("Unable to suscribe to Neovim event");

    let app = App::new();
    app.window.show_all();
    gtk::main();
}