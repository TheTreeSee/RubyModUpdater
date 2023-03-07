use std::fs::{self, File};
use std::io::Write;
use std::path::Path;
use std::process;

pub fn init_folder(folder_path: &Path) {
    if folder_path.exists() {
        println!("Mods folder already exists");
        return;
    }

    if let Err(e) = fs::create_dir_all(folder_path) {
        eprintln!("Error creating mods folder: {}", e);
        process::exit(1);
    }

    let mut file = File::create(folder_path.join("README.txt")).unwrap();
    file.write_all(b"Please put all mods you want in this file").unwrap();
}

pub fn version() { // todo add logic to change minecraft version
    println!("Useing minecraft version v1.19.3");
}

pub fn update() { // todo add logic to change minecraft version
    println!("Running update...");
}