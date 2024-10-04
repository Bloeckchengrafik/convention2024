use std::time::{Instant, SystemTime};
use crate::drivers::{DeviceDriver, DriverProcessError};
use messages::VrMessage;
use pub_sub::PubSub;

pub mod autodetect;
mod drivers;

pub struct InputDevices {
    pub headset_gyroscope: Option<drivers::headset_gyroscope::HeadsetGyroscopeDeviceDriver>,

    last_update: Instant
}

impl InputDevices {
    pub fn new(_bus: &PubSub<VrMessage>) -> InputDevices {
        autodetect::autodetect_input_devices()
    }

    pub fn process(&mut self, bus: &PubSub<VrMessage>) -> Result<(), Vec<DriverProcessError>> {
        let mut errors = Vec::new();
        if let Some(gyroscope) = &mut self.headset_gyroscope {
            if let Err(error) = gyroscope.process() {
                errors.push(error);
            }
        }

        if self.last_update.elapsed().as_secs() > 1 {
            self.last_update = Instant::now();
            bus.send(VrMessage::DriverStateUpdate {
                gyro_online: self.headset_gyroscope.is_some(),
                swarm_online: true,
                server_time: SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs(),
            }).unwrap();
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}