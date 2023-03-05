use serde::{Deserialize, Serialize};
use std::{env, fs};
use std::path::Path;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config { // todo look into this public shit
    pub name: String,
    pub version: String,
    pub path: String,
}

pub fn exists() -> bool {
    let app_data = env::var("APPDATA").unwrap();
    let app_folder = format!("{}/RubyModManager", app_data);
    let app_folder_path = Path::new(&app_folder);
    return app_folder_path.exists();
}

pub fn save_config(mods_folder_path: &Path) {
    let app_data = env::var("APPDATA").unwrap();
    let app_folder = format!("{}/RubyModManager", app_data);
    std::fs::create_dir_all(&app_folder).unwrap();

    let mods_folder_path = mods_folder_path.to_str().unwrap();

    let config = Config {
        name: String::from("RubyModManager"),
        version: String::from("0.1"),
        path: String::from(mods_folder_path),
    };

    let config_file = format!("{}/config.json", app_folder);
    let config_json = serde_json::to_string(&config).unwrap();
    fs::write(config_file, config_json).unwrap();
}


pub fn read_config() -> Config {
    let app_data = env::var("APPDATA").unwrap();
    let app_folder = format!("{}/RubyModManager", app_data);
    std::fs::create_dir_all(&app_folder).unwrap();

    // if app_folder.exists() {
    //     println!("Mods folder already exists");
    //     return;
    // }

    let config_file = format!("{}/config.json", app_folder);
    let config_json = fs::read_to_string(config_file).unwrap();
    let config: Config = serde_json::from_str(&config_json).unwrap();

    return config;
}