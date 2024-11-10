use etcetera::{choose_app_strategy, AppStrategy, AppStrategyArgs};
use mlua::{Lua, LuaSerdeExt};
use serde::{Deserialize, Serialize};
use std::{io::Read, sync::Mutex};
use tauri::{Manager, State};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Config {
    foo: u32,
    bar: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct AppData {
    config: Config,
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

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn test(state: State<'_, Mutex<AppData>>, st: String) -> Config {
    println!("{st:?}");
    return state.lock().expect("Error locking mutex").config.clone();
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let config = get_config();

    println!("{:?}", config);

    // TODO: Add menus here
    // - Settings should open new window?
    tauri::Builder::default()
        .setup(|app| {
            app.manage(Mutex::new(AppData { config }));

            Ok(())
        })
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![greet])
        .invoke_handler(tauri::generate_handler![test])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
