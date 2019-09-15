use std::path::Path;
use std::env;
use self::Theme::*;
use std::fs;
use dirs; 

pub enum Theme {
    Bootstrap
}

impl Theme {
    fn path(self) -> String {
        let home = env::var("HOME").unwrap_or_else(|_| panic!("Unable to locate home directory"));
        match self {
            Bootstrap => format!("{}/.config/illumination/themes/boostrap", home)
        }
    }

    pub fn rel_path(self) {

        let current = fs::canonicalize(Path::new("./")).expect("maeoimoireza");
        println!("{:?}", current);
        let abs_path = Path::new(&self.path());

    }
}
