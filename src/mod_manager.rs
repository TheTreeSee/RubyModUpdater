use std::fs::{self, File};
use std::io::Write;
use std::path::Path;
use std::process;

pub fn init_folder(folder_path: &Path) {
    println!("Mods folder path: {}", folder_path.display());

    if folder_path.exists() {
        println!("Mods folder already exists");
        return;
    }

    if let Err(e) = fs::create_dir_all(folder_path) {
        eprintln!("Error creating mods folder: {}", e);
        process::exit(1);
    }

    let mut file = File::create(folder_path.join("README.txt")).unwrap();
    file.write_all(b"This folder contains Minecraft mods.").unwrap();
}