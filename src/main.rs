use std::env;
use std::path::Path;
use std::process;

mod help;
mod config;
mod mod_manager;

fn main() {
    println!("");
    println!("------------------------------------------");
    println!("   Minecraft Mod Manager (Rust Edition)");
    println!("------------------------------------------");

    let mut args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        help::print_help();
        println!("------------------------------------------");
        println!("");
        process::exit(0);
    }

    args.remove(0);

    let mut need_help = false;

    let mut mods_folder_path = Path::new(".");
    println!("OG mods_folder_path: {:?}", mods_folder_path); //todo remove this and implement folder logic
    let config_data = config::read_config();
    mods_folder_path = Path::new(config_data.path.as_str());

    for (i, arg) in args.iter().enumerate() {
        let match_result = match arg.as_str() {
            "--help" | "-h" => {
                help::print_help();
                true
            }
            "--init" | "-i" => {
                mod_manager::init_folder(&mods_folder_path);
                true
            }
            "--folder" | "-f" => {
                if let Some(path_arg) = args.get(i + 1) {
                    mods_folder_path = Path::new(path_arg);
                    config::save_config(&mods_folder_path);
                } else {
                    println!("Mod folder path: {:?}", config_data.path);
                    println!("Tip: To change folter path, follow this commant with the new path.");
                }
                true
            }
            _ => {
                if need_help {
                    println!("Error: You must specify a valid option.");
                    help::print_help();
                }
                false
            }
        };

        if !match_result {
            need_help = true;
        }
    }
    println!("------------------------------------------");
    println!("");
}

