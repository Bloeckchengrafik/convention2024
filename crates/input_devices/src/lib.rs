use crate::drivers::swarm::VrSwarm;
use crate::drivers::{DeviceDriver, IdentifiedDeviceDriver};
use messages::{DriverState, VrMessage};
use pub_sub::PubSub;

pub mod autodetect;
mod drivers;

pub struct InputDevices {
    pub drivers: Vec<IdentifiedDeviceDriver>,

    swarm: Option<VrSwarm>,
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

    pub fn driver_states(&self) -> Vec<DriverState> {
        self.drivers.iter().map(|x| x.into()).collect()
    }
}