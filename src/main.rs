use std::env;
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;
use std::process;

mod help;
mod config;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        help::print_help();
        process::exit(0);
    }

    let mut mods_folder_path = Path::new(".");

    for (i, arg) in args.iter().enumerate() { // todo use a match statement
        if arg == "--help" || arg == "-h" {
            help::print_help();
            process::exit(0);
        } else if arg == "--init" || arg == "-i" {
            init_mods_folder(&mods_folder_path);
        } else if arg == "--folder" || arg == "-f" {
            if i == args.len() - 1 {
                eprintln!("Error: --folder argument requires a path");
                config::read_config(); //todo: print folder from file
                process::exit(1);
            } else {
                mods_folder_path = Path::new(&args[i + 1]);
                config::save_config(&mods_folder_path);
            }
        }
    }
}

fn init_mods_folder(mods_folder_path: &Path) {
    println!("Mods folder path: {}", mods_folder_path.display());

    if mods_folder_path.exists() {
        println!("Mods folder already exists");
        return;
    }

    if let Err(e) = fs::create_dir_all(mods_folder_path) {
        eprintln!("Error creating mods folder: {}", e);
        process::exit(1);
    }

    let mut file = File::create(mods_folder_path.join("README.txt")).unwrap();
    file.write_all(b"This folder contains Minecraft mods.").unwrap();
}

