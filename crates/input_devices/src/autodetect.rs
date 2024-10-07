use std::fmt::Debug;
use pub_sub::PubSub;
use serialport::SerialPortType;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use DeviceDriverType::HeadsetGyroscope;
use messages::VrMessage;
use crate::autodetect::DeviceDriverType::SteeringWheel;
use crate::drivers::headset::headset_gyroscope::HeadsetGyroscopeDeviceDriver;
use crate::drivers::{DeviceDriver, IdentifiedDeviceDriver};
use crate::drivers::swarm::steering_wheel::SteeringWheelDriver;
use crate::drivers::swarm::VrSwarm;
use crate::InputDevices;

#[derive(EnumIter, Debug)]
pub enum DeviceDriverType {
    HeadsetGyroscope,
    SteeringWheel,
}

impl DeviceDriverType {
    fn to_string(&self) -> String {
        format!("{:?}", self).to_string()
    }
}

struct AutodetectDeviceDriverList {
    drivers: Vec<IdentifiedDeviceDriver>,
}

impl AutodetectDeviceDriverList {
    fn new() -> AutodetectDeviceDriverList {
        AutodetectDeviceDriverList {
            drivers: Vec::new()
        }
    }

    fn push(&mut self, kind: DeviceDriverType, driver: Box<dyn DeviceDriver + Send>) {
        // Check if the driver is already in the list, if so, replace it
        let name = kind.to_string();
        let found = self.drivers.iter_mut().find(|x| x.name == name);
        if let Some(driver) = found {
            driver.driver = Some(driver.driver.take().unwrap());
        } else {
            self.drivers.push(IdentifiedDeviceDriver {
                driver: Some(driver),
                name,
            });
        }
    }

    fn finish(self) -> Vec<IdentifiedDeviceDriver> {
        let mut drivers = self.drivers;
        DeviceDriverType::iter().for_each(|kind| {
            if !drivers.iter().any(|x| x.name == kind.to_string()) {
                drivers.push(IdentifiedDeviceDriver {
                    driver: None,
                    name: kind.to_string(),
                });
            }
        });

        drivers.sort_by(|a, b| a.name.cmp(&b.name));
        drivers
    }
}

pub async fn autodetect_input_devices(bus: &PubSub<VrMessage>) -> InputDevices {
    let mut drivers = AutodetectDeviceDriverList::new();
    let mut swarm: Option<VrSwarm> = None;

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

            drivers.push(HeadsetGyroscope, HeadsetGyroscopeDeviceDriver::new(port, &bus));
        } else if port_vendor == "1a86" { // CH341 USB to serial (ftSwarm)
            let vr_swarm = VrSwarm::new(&port_name).await;

            drivers.push(SteeringWheel, SteeringWheelDriver::new(&vr_swarm, &bus));

            swarm = Some(vr_swarm);
        }
    }

    InputDevices {
        swarm,
        drivers: drivers.finish(),
        last_update: std::time::Instant::now(),
        bus: bus.clone(),
    }
}