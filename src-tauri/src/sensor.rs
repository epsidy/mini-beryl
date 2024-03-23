use std::cmp;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;
use crossbeam_channel::{Receiver, Sender};
use crossbeam_utils::sync::WaitGroup;
use serialport::SerialPort;
use tauri::Manager;
use crate::utils::hardware::{clear_buffer, DModuleCommand};
use crate::utils::processing::bytes_to_physical_normal;

const REFRESH_SIZE: usize = 10;

pub fn sensor_task(
    mut sensor: Box<dyn SerialPort>,
    sender: Sender<Vec<f32>>,
    running: Arc<AtomicBool>,
    wg: WaitGroup,
) {
    let mut package = [0u8; 1024];
    let buffer_size = 1024 * REFRESH_SIZE;
    let mut buffer = Vec::with_capacity(buffer_size);

    sensor.write(&[DModuleCommand::EcgPowOff as u8]).unwrap_or(0);
    clear_buffer(&sensor);
    sensor.write(&[DModuleCommand::Ecg3PowOn as u8]).unwrap_or(0);

    if let Err(_) = sensor.read_exact(&mut package) {
        sensor.write(&[DModuleCommand::EcgPowOff as u8]).unwrap_or(0);
        return;
    }

    sensor.set_timeout(Duration::from_millis(16)).unwrap_or(());

    drop(wg);

    while running.load(Ordering::Relaxed) {
        match sensor.read_exact(&mut package) {
            Ok(_) => {
                buffer.extend_from_slice(&package);
                if buffer.len() == buffer_size {
                    let points = bytes_to_physical_normal(&buffer, REFRESH_SIZE);
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


pub fn data_aggregation_process(
    receiver: Receiver<Vec<f32>>,
    running: Arc<AtomicBool>,
    app: tauri::AppHandle,
    wg: WaitGroup,
    grid_nums: usize,
) {
    const ERASE_SIZE: usize = 35;
    let mut window_buffer = vec![f32::NAN; 500 * grid_nums];
    let mut current: usize = 0;
    wg.wait();
    while running.load(Ordering::Relaxed) {
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
                app.emit_all("sensor", data).unwrap();
            }
            Err(_) => break
        };
    }
    running.store(false, Ordering::Relaxed);
}
