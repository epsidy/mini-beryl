use std::io::Write;
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::Duration;
use crossbeam_channel::bounded;
use crossbeam_utils::sync::WaitGroup;
use serialport::{available_ports, SerialPort, SerialPortInfo};
use serialport::SerialPortType::UsbPort;

use crate::modes::normal::{data_aggregation_process, ecg_task};
use crate::modes::hall_effect::hall_effect_task;
use crate::utils::hardware::check_sensor;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct SensorMode {
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
pub fn start_normal_mode(payload: SensorMode, app: tauri::AppHandle, running: tauri::State<Arc<AtomicBool>>) -> bool {
    running.inner().store(true, Ordering::Relaxed);

    return match serialport::new(&payload.sensor, 3_000_000).timeout(Duration::from_millis(500)).open() {
        Ok(port) => {
            let wg = WaitGroup::new();
            let wg_clone = wg.clone();
            let app_clone = app.clone();
            let running_sensor = running.inner().clone();
            let running_data = running.inner().clone();
            let (sender, receiver) = bounded::<Vec<f32>>(1000);
            thread::spawn(move || ecg_task(port, sender, running_sensor, wg));
            thread::spawn(move || data_aggregation_process(receiver, running_data, app_clone, wg_clone, 3));
            true
        }
        Err(_) => false
    };
}


#[tauri::command]
pub fn stop(
    running: tauri::State<Arc<AtomicBool>>,
    open_port: tauri::State<Mutex<Option<Box<dyn SerialPort>>>>
) {
    running.store(false, Ordering::Relaxed);
    let mut open_port_state = open_port.inner().lock().unwrap();
    *open_port_state = None;
}


#[tauri::command]
pub fn start_hall_mode(
    payload: SensorMode,
    app: tauri::AppHandle,
    running: tauri::State<Arc<AtomicBool>>,
    open_port: tauri::State<Mutex<Option<Box<dyn SerialPort>>>>,
) -> bool {
    running.inner().store(true, Ordering::Relaxed);
    return match serialport::new(&payload.sensor, 3_000_000).timeout(Duration::from_millis(500)).open() {
        Ok(port) => {
            let port_clone = port.try_clone().unwrap();
            let mut open_port_state = open_port.inner().lock().unwrap();
            *open_port_state = Some(port_clone);

            let app_clone = app.clone();
            let running_clone = running.inner().clone();
            thread::spawn(move || hall_effect_task(port, running_clone, app_clone));
            true
        }
        Err(_) => false
    };
}

#[tauri::command]
pub fn send_command(
    payload: String,
    open_port: tauri::State<Mutex<Option<Box<dyn SerialPort>>>>,
) {
    let command: u8 = match payload.as_str() {
        "CMD_InfoSystem" => 0x00,
        "CMD_ID" => 0x00,
        "CMD_UID" => 0x01,
        "CMD_SID" => 0x02,
        "CMD_HID" => 0x03,
        "CMD_ECG" => 0x10,
        "CMD_ECG_PWR_OFF" => 0x10,
        "CMD_ECG3_PWR_ON" => 0x11,
        "CMD_IMU" => 0x20,
        "CMD_IMU_PWR_OFF" => 0x20,
        "CMD_IMU_PWR_ON" => 0x21,
        "CMD_HALL" => 0x30,
        "CMD_HALL_PWR_OFF" => 0x30,
        "CMD_HALL_PWR_ON" => 0x31,
        "CMD_HALL_B0_PWR_OFF" => 0x32,
        "CMD_HALL_B0_PWR_ON" => 0x33,
        "CMD_HALL_GRD_PWR_OFF" => 0x34,
        "CMD_HALL_GRD_PWR_ON" => 0x35,
        _ => 0x10
    };
    let mut open_port_state = open_port.inner().lock().unwrap();
    match &mut *open_port_state {
        None => {}
        Some(port) => { port.write(&[command]).unwrap(); }
    };
}
