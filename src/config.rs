use serde::{Deserialize, Serialize};
use std::{env, fs};
use std::path::Path;

#[derive(Debug, Serialize, Deserialize)]
struct Config<'a> {
    name: String,
    version: String,
    path: &'a Path, // todo make this work
}

pub fn save_config(mods_folder_path: &Path) {
    let app_data = env::var("APPDATA").unwrap();
    let app_folder = format!("{}/RubyModManager", app_data);
    std::fs::create_dir_all(&app_folder).unwrap();

    let config = Config {
        name: String::from("My App"),
        version: String::from("1.0"),
        path: mods_folder_path,
    };

    let config_file = format!("{}/config.json", app_folder);
    let config_json = serde_json::to_string(&config).unwrap();
    fs::write(config_file, config_json).unwrap();
}


pub fn read_config() {
    let app_data = env::var("APPDATA").unwrap();
    let app_folder = format!("{}/RubyModManager", app_data);
    std::fs::create_dir_all(&app_folder).unwrap();

    let config_file = format!("{}/config.json", app_folder);
    let config_json = fs::read_to_string(config_file).unwrap();
    let config: Config = serde_json::from_str(&config_json).unwrap();

    println!("Config: {:?}", config);
}