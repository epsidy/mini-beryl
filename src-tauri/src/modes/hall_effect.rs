use std::{cmp, io};
use std::io::Read;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;
use crossbeam_channel::{Receiver, Sender};
use crossbeam_utils::sync::WaitGroup;
use serialport::{ClearBuffer, SerialPort};
use tauri::Manager;
use crate::global::REFRESH_SIZE;
use crate::utils::hardware::{clear_buffer, DModuleCommand};
use crate::utils::processing::{bytes_to_physical_hall, bytes_to_physical_normal};

// pub fn hall_effect_task(
//     mut sensor: Box<dyn SerialPort>,
//     running: Arc<AtomicBool>,
//     app: tauri::AppHandle,
// ) {
//     let mut buffer = [0u8; 1024];
//
//     sensor.write(&[DModuleCommand::EcgPowOff as u8]).unwrap_or(0);
//     sensor.clear(ClearBuffer::All).unwrap();
//
//     while running.load(Ordering::Acquire) {
//         match sensor.read(&mut buffer) {
//             Ok(size) => {
//                 if size > 0 {
//                     if size >= 45 {
//                         sensor.write(&[DModuleCommand::EcgPowOff as u8]).unwrap_or(0);
//                         let mut string = String::with_capacity(1080);
//                         for (i, &byte) in buffer.iter().take(cmp::min(size, 80)).enumerate() {
//                             if i != 0 && i % 8 == 0 {
//                                 string.push_str("-------- 8\n");
//                             }
//                             if i != 0 && i % 20 == 0 {
//                                 string.push_str("-------- 20\n");
//                             }
//                             string.push_str(&format!("{:08b}\n", byte));
//                         }
//                         app.emit_all("hall", string).unwrap()
//                     } else {
//                         let mut hex_string = hex::encode_upper(&buffer[..size]);
//                         hex_string.push_str("\n");
//                         app.emit_all("hall", hex_string).unwrap()
//                     }
//                     sensor.clear(ClearBuffer::All).unwrap();
//                 }
//             }
//             Err(ref e) if e.kind() == io::ErrorKind::TimedOut => {}
//             Err(_) => break
//         }
//     }
//     sensor.write(&[DModuleCommand::EcgPowOff as u8]).unwrap_or(0);
// }


pub fn hall_effect_task(
    mut sensor: Box<dyn SerialPort>,
    running: Arc<AtomicBool>,
    sender: Sender<Vec<f32>>,
    wg: WaitGroup,
) {
    let mut package = [0u8; 2560];
    let buffer_size = 2560 * REFRESH_SIZE;
    let mut buffer = Vec::with_capacity(buffer_size);

    sensor.write(&[DModuleCommand::EcgPowOff as u8]).unwrap_or(0);
    sensor.clear(ClearBuffer::All).unwrap();
    sensor.write(&[DModuleCommand::Ecg3PowOn as u8]).unwrap_or(0);

    if let Err(_) = sensor.read_exact(&mut package) {
        sensor.write(&[DModuleCommand::EcgPowOff as u8]).unwrap_or(0);
        return;
    }

    sensor.set_timeout(Duration::from_millis(16)).unwrap_or(());

    drop(wg);

    while running.load(Ordering::Acquire) {
        match sensor.read_exact(&mut package) {
            Ok(_) => {
                buffer.extend_from_slice(&package);
                if buffer.len() == buffer_size {
                    let points = bytes_to_physical_hall(&buffer, REFRESH_SIZE);
                    if let Err(_) = sender.try_send(points) {
                        break;
                    }
                    buffer.clear();
                }
            }
            Err(_) => break
        }
    }
    sensor.write(&[DModuleCommand::EcgPowOff as u8]).unwrap_or(0);
}


pub fn hall_effect_data_aggregation_process(
    receiver: Receiver<Vec<f32>>,
    running: Arc<AtomicBool>,
    app: tauri::AppHandle,
    wg: WaitGroup,
) {
    const ERASE_SIZE: usize = 35;
    let mut window_buffer = vec![f32::NAN; 500 * 3];
    let mut current: usize = 0;
    wg.wait();
    while running.load(Ordering::Acquire) {
        match receiver.recv_timeout(Duration::from_millis(160)) {
            Ok(new) => {
                new.chunks(10).enumerate().for_each(|(i, lead)| {
                    let boundary = (i + 1) * 500;
                    let index = current + i * 500;

                    window_buffer[index..index + REFRESH_SIZE].copy_from_slice(lead);

                    let erase_start = cmp::min(index + REFRESH_SIZE, boundary);
                    let erase_end = cmp::min(index + REFRESH_SIZE + ERASE_SIZE, boundary);
                    window_buffer[erase_start..erase_end].fill(f32::NAN);
                });
                current = (current + REFRESH_SIZE) % 500;
                let data = window_buffer
                    .chunks(500)
                    .map(|chunk| chunk.to_vec())
                    .collect::<Vec<Vec<f32>>>();
                app.emit_all("hall", data).unwrap();
            }
            Err(_) => break
        };
    }
    running.store(false, Ordering::Release);
}
