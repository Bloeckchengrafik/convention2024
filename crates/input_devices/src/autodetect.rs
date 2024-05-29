use crate::drivers::headset_gyroscope::HeadsetGyroscopeDeviceDriver;
use crate::InputDevices;

pub fn autodetect_input_devices() -> InputDevices {
    let mut headset_gyroscope = None;

    let ports = serialport::available_ports().unwrap();
    for port in ports {
        let port_name = &port.port_name;
        if (!port_name.contains("ttyUSB") && !port_name.contains("ttyACM")) {
            continue;
        }
        let mut port = ftswarm_serial::SerialCommunication::connect(port_name);

        if (HeadsetGyroscopeDeviceDriver::check_discovery_signature(&mut port)){
            headset_gyroscope = Some(HeadsetGyroscopeDeviceDriver::new(port));
        } else {
            continue;
        }
    }

    InputDevices {
        headset_gyroscope: headset_gyroscope.unwrap(),
    }
}