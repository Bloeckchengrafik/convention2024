use crate::drivers::swarm::VrSwarm;
use crate::drivers::{DeviceDriver, IdentifiedDeviceDriver};
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
    pub async fn build(&mut self) -> Vec<Box<dyn DeviceDriver>> {
        let mut drivers = Vec::new();

        while let Some(driver) = self.drivers.pop() {
            if let Some(driver) = driver.driver {
                drivers.push(driver().await);
            }
        }

        drivers
    }
}

impl InputDevices {
    pub async fn new(bus: &PubSub<VrMessage>) -> InputDevices {
        autodetect::autodetect_input_devices(bus).await
    }

    pub async fn process(&mut self) {
        if self.last_update.elapsed().as_secs() > 1 {
            self.last_update = Instant::now();
            let _ = self.bus.send(VrMessage::DriverStateUpdate {
                states: self.drivers.iter().map(|x| x.into()).collect()
            });
        }
    }
}