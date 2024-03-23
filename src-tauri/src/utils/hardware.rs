use serialport::{ClearBuffer, SerialPort};

/**
D module command
 **/
pub enum DModuleCommand {
    EcgPowOff = 0x10,
    Ecg3PowOn = 0x11,
}


pub fn clear_buffer(port: &Box<dyn SerialPort>) {
    port.clear(ClearBuffer::All).unwrap_or(());
}

pub fn check_sensor(port: &mut Box<dyn SerialPort>) -> bool {
    const D_MODULE_ID: &str = "FFF0800CFF";

    let id = get_hardware_info(port);

    match id.as_str() {
        D_MODULE_ID => true,
        _ => {
            port.write(&[DModuleCommand::EcgPowOff as u8]).unwrap_or(0);
            let id = get_hardware_info(port);
            match id.as_str() {
                D_MODULE_ID => true,
                _ => false
            }
        }
    }
}

fn get_hex_info(info_buffer: &[u8], mut hex_string: String) -> String {
    info_buffer
        .into_iter()
        .for_each(|byte| hex_string.push_str(&format!("{:02X?}", byte)));
    hex_string
}

pub fn get_hardware_info(port: &mut Box<dyn SerialPort>) -> String {
    let mut buffer = vec![0u8; 5];

    clear_buffer(port);
    port.write(&[0x00u8]).unwrap_or(0);
    port.read_exact(&mut buffer).unwrap_or(());

    get_hex_info(&buffer, String::with_capacity(10))
}