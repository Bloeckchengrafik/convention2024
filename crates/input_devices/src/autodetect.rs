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
        let mut port = serialport::new(port_name, 115200)
            .timeout(std::time::Duration::from_millis(10))
            .open()
            .expect(format!("Failed to open serial port at {}", port_name).as_str());

        if HeadsetGyroscopeDeviceDriver::check_discovery_signature(&mut port) {
            headset_gyroscope = Some(HeadsetGyroscopeDeviceDriver::new(port));
        } else {
            continue;
        }
    }

    InputDevices {
        headset_gyroscope: headset_gyroscope.unwrap(),
    }
}