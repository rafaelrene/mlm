// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::io::Read;

use etcetera::{choose_app_strategy, AppStrategy, AppStrategyArgs};

fn main() {
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
        .open(&config_file_path)
        .expect("Could not load config file!");

    let mut config_file_contents: String = "".to_string();
    file.read_to_string(&mut config_file_contents)
        .expect("Config file could not be read!");

    println!("Config: {config_file_path:?}");
    println!("File: {file:?}");
    println!("Content: {config_file_contents:?}");

    mlm_lib::run()
}
