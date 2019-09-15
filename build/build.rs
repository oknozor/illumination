use dirs;
use std::fs;
use std::process::Command;

fn main() {
    println!("Starting to build Illumination.");
    println!("Fetching default themes.");

    Command::new("sh")
        .args(&["build/themes.sh"])
        .status()
        .expect("Error fetching default themes");

    let illumination_config_dir = format!(
        "{}/illumination/",
        dirs::config_dir().unwrap().to_str().unwrap()
    );

    println!("Copying themes to user config dir.");
    fs::create_dir_all(&illumination_config_dir).unwrap();

    Command::new("cp")
        .args(&["-R", "themes", &illumination_config_dir])
        .status()
        .expect("Error moving themes to user config dir");
}
