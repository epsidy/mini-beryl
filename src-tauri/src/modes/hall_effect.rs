use std::io;
use std::io::Read;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
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

    while running.load(Ordering::Relaxed) {
        match sensor.read(&mut buffer) {
            Ok(size) => {
                if size > 0 {
                    if size >= 500 {
                        sensor.write(&[DModuleCommand::EcgPowOff as u8]).unwrap_or(0);
                        let mut string = String::with_capacity(1080);
                        for (i, &byte) in buffer.iter().take(100).enumerate() {
                            if i !=0 && i % 8 == 0 {
                                string.push_str("-------- 8\n");
                            }
                            if i !=0 && i % 20 == 0 {
                                string.push_str("-------- 20\n");
                            }
                            string.push_str(&format!("{:08b}\n", byte));
                        }
                        app.emit_all("hall", string).unwrap()
                    } else {
                        let mut hex_string = hex::encode_upper(&buffer[..size]);
                        hex_string.push_str("\n");
                        app.emit_all("hall", hex_string).unwrap()
                    }
                }
                clear_buffer(&sensor);
            }
            Err(ref e) if e.kind() == io::ErrorKind::TimedOut => {}
            Err(_) => break
        }
    }
    sensor.write(&[DModuleCommand::EcgPowOff as u8]).unwrap_or(0);
}


