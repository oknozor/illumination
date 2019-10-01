use std::fs::File;
use std::io::prelude::*;

fn get_rust_doc_uri(path: &str) -> Result<String, std::io::Error> {
    let mut contents = String::new();
    let mut file = File::open(path)?;
    file.read_to_string(&mut contents)?;
    Ok(contents)
}
