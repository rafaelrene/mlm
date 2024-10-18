// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

use etcetera::{choose_app_strategy, AppStrategy, AppStrategyArgs};
use mlua::{Lua, LuaSerdeExt};
use serde::{Deserialize, Serialize};
use std::io::Read;

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    foo: u32,
    bar: String,
}

fn get_config() -> Config {
    let strategy = choose_app_strategy(AppStrategyArgs {
        top_level_domain: "org".to_string(),
        author: "Rene Rafael".to_string(),
        app_name: "mlm".to_string(),
    })
    .expect("Strategy failed!");

    let config_file_path = strategy.in_config_dir("config.lua");

    std::fs::create_dir_all(strategy.config_dir()).expect("Could not create config directory!");
    let mut file = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .read(true)
        .open(config_file_path)
        .expect("Could not load config file!");

    let mut config_file_contents: String = "".to_string();
    file.read_to_string(&mut config_file_contents)
        .expect("Config file could not be read!");

    let lua = Lua::new();
    let chunk = lua.load(config_file_contents);
    let config: Config = lua
        .from_value(chunk.eval().expect("Could not parse config.lua!"))
        .expect("Could not deserialize config.lua!");

    return config;
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let config = get_config();

    println!("{:?}", config);

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
