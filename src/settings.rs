use config::{Config, File};
use lazy_static;
use std::collections::HashMap;
use std::sync::RwLock;

lazy_static! {
    static ref SETTINGS: RwLock<Config> = RwLock::new({
        let mut settings = Config::default();
        settings.merge(File::with_name("config.toml")).unwrap();

        settings
    });
}

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
