// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::{Arc, Mutex};
use std::sync::atomic::AtomicBool;
use serialport::SerialPort;
use crate::commands::{scan_sensors, start_normal_mode, stop, start_hall_effect_mode};

mod utils;
mod modes;
mod commands;
mod global;


// Learn more about Tauri commands.rs at https://tauri.app/v1/guides/features/command

fn main() {
    tauri::Builder::default()
        .manage(Mutex::new(None::<Box<dyn SerialPort>>))
        .manage(Arc::new(AtomicBool::new(false)))
        .invoke_handler(tauri::generate_handler![
            scan_sensors,
            start_normal_mode,
            start_hall_effect_mode,
            stop,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

