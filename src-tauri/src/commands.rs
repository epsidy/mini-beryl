use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::Duration;
use crossbeam_channel::bounded;
use crossbeam_utils::sync::WaitGroup;
use serialport::{available_ports, SerialPortInfo};
use serialport::SerialPortType::UsbPort;
use crate::modes::normal::{data_aggregation_process, sensor_task};
use crate::utils::hardware::check_sensor;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
struct SensorMode {
    sensor: String,
    mode: String,
}

#[tauri::command]
pub async fn scan_sensors() -> Vec<String> {
    let mut sensors = Vec::<String>::new();
    if let Ok(ports) = available_ports() {
        let ports = ports
            .into_iter()
            .filter(|port| match port.port_type {
                UsbPort(..) => !port.port_name.starts_with("/dev/cu"),
                _ => false
            })
            .collect::<Vec<SerialPortInfo>>();
        for port in ports {
            let path = port.port_name;
            if let Ok(mut opened_port) = serialport::new(&path, 3_000_000)
                .timeout(Duration::from_millis(200))
                .open() {
                if check_sensor(&mut opened_port) {
                    sensors.push(path)
                }
            }
        }
    }
    sensors
}

#[tauri::command]
pub fn start(payload: SensorMode, app: tauri::AppHandle, running: tauri::State<Arc<AtomicBool>>) -> bool {
    running.store(true, Ordering::Relaxed);

    let grid_nums = match payload.mode.as_str() {
        "normal" => 3,
        "hall" => 5,
        _ => 3,
    };

    return match serialport::new(&payload.sensor, 3_000_000).timeout(Duration::from_millis(500)).open() {
        Ok(port) => {
            let wg = WaitGroup::new();
            let wg_clone = wg.clone();
            let app_clone = app.clone();
            let running_sensor = running.inner().clone();
            let running_data = running.inner().clone();
            let (sender, receiver) = bounded::<Vec<f32>>(1000);
            thread::spawn(move || sensor_task(port, sender, running_sensor, wg));
            thread::spawn(move || data_aggregation_process(receiver, running_data, app_clone, wg_clone, grid_nums));
            true
        }
        Err(_) => false
    };
}


#[tauri::command]
pub fn stop(running: tauri::State<Arc<AtomicBool>>) {
    running.store(false, Ordering::Relaxed);
}
