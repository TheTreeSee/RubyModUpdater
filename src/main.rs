use std::env;
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        print_help();
        process::exit(0);
    }

    let mut mods_folder_path = Path::new(".");

    for (i, arg) in args.iter().enumerate() {
        if arg == "--help" || arg == "-h" {
            print_help();
            process::exit(0);
        } else if arg == "--init" || arg == "-i" {
            init_mods_folder(&mods_folder_path);
        } else if arg == "--folder" || arg == "-f" {
            if i == args.len() - 1 {
                eprintln!("Error: --folder argument requires a path");
                process::exit(1);
            } else {
                mods_folder_path = Path::new(&args[i + 1]);
                save_config();
            }
        }
    }
}

fn print_help() {
    println!("Usage: minecraft-mod-manager [OPTIONS]");
    println!();
    println!("OPTIONS:");
    println!("  -h, --help      Print this help message");
    println!("  -i, --init      Create the mods folder if it doesn't exist");
    println!("  -f, --folder    Set the path to the mods folder (default: current directory)");
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

fn save_config() {
    let app_data = env::var("APPDATA").unwrap();
    let app_folder = format!("{}/my-app", app_data);
    // create the folder if it doesn't exist
    std::fs::create_dir_all(&app_folder).unwrap();
    // store your configuration files in the app_folder
}