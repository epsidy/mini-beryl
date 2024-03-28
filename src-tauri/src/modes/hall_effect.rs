use std::io;
use std::io::Read;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use serialport::FlowControl::Hardware;
use serialport::SerialPort;
use tauri::Manager;
use crate::utils::hardware::{clear_buffer, DModuleCommand};

pub fn hall_effect_task(
    mut sensor: Box<dyn SerialPort>,
    running: Arc<AtomicBool>,
    app: tauri::AppHandle,
) {
    let mut buffer = [0u8; 4096];
    sensor.write(&[DModuleCommand::EcgPowOff as u8]).unwrap_or(0);
    clear_buffer(&sensor);
    sensor.write(&[0x01u8]).unwrap_or(0);

    while running.load(Ordering::Relaxed) {
        match sensor.read_exact(&mut buffer) {
            Ok(()) => {
                let hex_string = hex::encode_upper(&buffer);
                app.emit_all("hall", hex_string).unwrap()
            }
            Err(ref e) if e.kind() == io::ErrorKind::TimedOut => {
                let hex_string = hex::encode_upper(&buffer);
                println!("{}", hex_string);
            },
            Err(_) => break
        }
    }
    sensor.write(&[DModuleCommand::EcgPowOff as u8]).unwrap_or(0);
}


