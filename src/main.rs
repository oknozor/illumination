extern crate gtk;

#[macro_use]
extern crate horrorshow;

use std::{process, thread};

use gtk::*;
use webkit2gtk::WebContext;
use webkit2gtk::WebView;

pub mod ui;
pub mod nvim;
pub mod preview;

use ui::App;
use crate::nvim::handler::{NvimHandler, BufferHandler};
use neovim_lib::{Session, Neovim};
use neovim_lib::neovim_api::NeovimApi;
use std::sync::mpsc;
use std::fs::File;
use std::io::Write;

fn main() {
    /*
        let context = WebContext::get_default().unwrap();
        let webview = WebView::new_with_context(&context);
        webview.load_html(&html, None);
    */

    println!("initializing app");
    let app = App::new();
    app.window.show_all();
    app.connect_events();
    gtk::main();
}