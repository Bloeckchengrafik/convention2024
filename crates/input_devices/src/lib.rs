use crate::drivers::swarm::VrSwarm;
use crate::drivers::{DeviceDriver, DriverProcessError, IdentifiedDeviceDriver};
use messages::VrMessage;
use pub_sub::PubSub;
use std::time::Instant;

pub mod autodetect;
mod drivers;

pub struct InputDevices {
    pub drivers: Vec<IdentifiedDeviceDriver>,

    swarm: Option<VrSwarm>,
    last_update: Instant,
    bus: PubSub<VrMessage>,
}

impl InputDevices {
    pub async fn new(bus: &PubSub<VrMessage>) -> InputDevices {
        autodetect::autodetect_input_devices(bus).await
    }

    pub async fn process(&mut self) -> Result<(), Vec<DriverProcessError>> {
        let mut errors = Vec::new();

        for mut driver in &mut self.drivers {
            if let Err(error) = driver.process().await {
                errors.push(error);
            }
        }

        if self.last_update.elapsed().as_secs() > 1 {
            self.last_update = Instant::now();
            self.bus.send(VrMessage::DriverStateUpdate {
                states: self.drivers.iter().map(|x| x.into()).collect()
            }).unwrap();
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}