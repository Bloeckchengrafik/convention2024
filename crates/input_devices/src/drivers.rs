use messages::DriverState;
use crate::autodetect::BuildableDriver;

pub mod swarm;
pub mod headset;

#[derive(Debug)]
pub enum DriverProcessError {
    DataframeError(String),
    IoError,
    BusError,
    SwarmError,
}

#[async_trait::async_trait]
pub trait DeviceDriver {
    async fn process(&mut self) -> Result<(), DriverProcessError>;
}

pub struct IdentifiedDeviceDriver {
    pub driver: Option<BuildableDriver>,
    pub name: String,
}
impl IdentifiedDeviceDriver {
    pub fn into(&self) -> DriverState {
        if self.driver.is_some() {
            DriverState::Online {
                name: self.name.clone(),
            }
        } else {
            DriverState::Offline {
                name: self.name.clone(),
            }
        }
    }
}