use std::process::Command;

pub fn get_uri() -> Result<String, std::io::Error> {
    Command::new("rustup")
        .args(&["doc", "--path"])
        .output()
        .map(|out| String::from_utf8(out.stdout).expect("unable to convert doc path to valid utf8"))
}
