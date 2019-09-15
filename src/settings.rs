use config::{Config, File as ConfigFile};
use lazy_static;
use std::collections::HashMap;
use std::sync::RwLock;
use std::fs::File;
use std::io::prelude::*;
use std::env;

lazy_static! {
    static ref SETTINGS: RwLock<Config> = RwLock::new({
        let mut settings = Config::default();
        settings.merge(ConfigFile::with_name("config.toml")).unwrap();

        settings
    });

    // Unfortunatly is seems webkit does accept any href from the file system, to hack our way around this we just preload css and hljs
    // see : https://github.com/gtk-rs/webkit2gtk-rs/issues/56
    pub static ref THEME: String = {
        let home = env::var("HOME").unwrap_or_else(|_| panic!("Unable to locate home directory"));
        let mut style = File::open(&format!("{}/.config/illumination/themes/default/style.css", home)).expect("Error opening default style.css");
        let mut hljs_css = File::open(&format!("{}/.config/illumination/themes/default/hljs.min.css", home)).expect("Error opening hljs.min.css");
        let mut contents = String::new();
        style.read_to_string(&mut contents).expect("Unable to write css theme");
        hljs_css.read_to_string(&mut contents).expect("Unable to write css theme");
        contents
    };

    pub static ref JS: String = {
        let home = env::var("HOME").unwrap_or_else(|_| panic!("Unable to locate home directory"));
        let mut hljs = File::open(format!("{}/.config/illumination/themes/default/hljs.js", home)).expect("Error opening hljs.js");
        let mut hljs_rust = File::open(format!("{}/.config/illumination/themes/default/hljs-rust.js", home)).expect("Error opening rust hljs-rust.js");
        let mut contents = String::new();
        hljs.read_to_string(&mut contents).expect("Unable to write hljs");
        hljs_rust.read_to_string(&mut contents).expect("Unable to write hljs rust");
        contents
    };
}

// dump config.toml
#[cfg(debug_assertions)]
pub fn show() {
    info!(
        " * Settings :: \n\x1b[31m{:?}\x1b[0m",
        SETTINGS
            .read()
            .unwrap()
            .clone()
            .try_into::<HashMap<String, String>>()
            .unwrap()
    );
}
