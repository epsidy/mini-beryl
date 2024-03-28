// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use crate::commands::{scan_sensors, start_hall_mode, start_normal_mode, stop};

mod utils;
mod modes;
mod commands;



// Learn more about Tauri commands.rs at https://tauri.app/v1/guides/features/command

fn main() {
    tauri::Builder::default()
        .manage(Arc::new(AtomicBool::new(false)))
        .invoke_handler(tauri::generate_handler![scan_sensors, start_normal_mode, start_hall_mode, stop])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
