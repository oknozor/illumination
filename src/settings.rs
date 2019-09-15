use config::{Config, File as ConfigFile};
use dirs;
use lazy_static;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::sync::RwLock;

lazy_static! {
    static ref SETTINGS: RwLock<Config> = RwLock::new({
        let mut settings = Config::default();
        settings.merge(ConfigFile::with_name("config.toml")).unwrap();

        settings
    });

    // Unfortunatly is seems webkit does accept any href from the file system, to hack our way around this we just preload css and hljs
    // see : https://github.com/gtk-rs/webkit2gtk-rs/issues/56
    pub static ref THEME: String = {
        let config_dir = dirs::config_dir().unwrap();
        let config_dir = config_dir
            .to_str()
            .unwrap();

        let mut style = File::open(&format!("{}/illumination/themes/default/style.css", config_dir)).expect("Error opening default style.css");
        let mut hljs_css = File::open(&format!("{}/illumination/themes/default/hljs.min.css", config_dir)).expect("Error opening hljs.min.css");
        let mut contents = String::new();
        style.read_to_string(&mut contents).expect("Unable to write css theme");
        hljs_css.read_to_string(&mut contents).expect("Unable to write css theme");
        contents
    };

    pub static ref JS: String = {
        let config_dir = dirs::config_dir().unwrap();
        let config_dir = config_dir
            .to_str()
            .unwrap();

        let mut hljs = File::open(format!("{}/illumination/themes/default/hljs.min.js", config_dir)).expect("Error opening hljs.min.js");
        let mut hljs_rust = File::open(format!("{}/illumination/themes/default/hljs-rust.js", config_dir)).expect("Error opening rust hljs-rust.js");
        let mut contents = String::new();
        hljs.read_to_string(&mut contents).expect("Unable to write hljs");
        hljs_rust.read_to_string(&mut contents).expect("Unable to write hljs rust");
        contents
    };

    pub static ref THEME_DIR: String = {

        let config_dir = dirs::config_dir().unwrap();
        let config_dir = config_dir
            .to_str()
            .unwrap();
        format!("{}/themes", config_dir)
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
