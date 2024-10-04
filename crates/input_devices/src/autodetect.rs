use serialport::SerialPortType;
use crate::drivers::headset_gyroscope::HeadsetGyroscopeDeviceDriver;
use crate::InputDevices;

pub fn autodetect_input_devices() -> InputDevices {
    let mut headset_gyroscope = None;

    let ports = serialport::available_ports().unwrap();
    for port in ports {
        let port_name = &port.port_name;
        if !port_name.contains("ttyUSB") && !port_name.contains("ttyACM") {
            continue;
        }

        let port_vendor = match port.port_type {
            SerialPortType::UsbPort(info) => { info.manufacturer.unwrap_or("".to_string()) }
            _ => continue,
        };

        if port_vendor == "Espressif" {
            let port = serialport::new(port_name, 115200)
                .timeout(std::time::Duration::from_millis(10))
                .open()
                .expect(format!("Failed to open serial port at {}", port_name).as_str());
            headset_gyroscope = Some(HeadsetGyroscopeDeviceDriver::new(port));
        } else if port_vendor == "1a86" { // CH341 USB to serial (ftSwarm)
            continue;
        } else {
            continue;
        }
    }

    InputDevices {
        headset_gyroscope,

        last_update: std::time::Instant::now()
    }
}