use async_trait::async_trait;
use messages::DriverState;

pub mod swarm;
pub mod headset;

#[derive(Debug)]
pub enum DriverProcessError {
    DataframeError(String),
    IoError,
    BusError
}

#[async_trait::async_trait]
pub trait DeviceDriver {
    async fn process(&mut self) -> Result<(), DriverProcessError>;
}

pub struct IdentifiedDeviceDriver {
    pub driver: Option<Box<dyn DeviceDriver + Send>>,
    pub name: String,
}

#[async_trait]
impl DeviceDriver for IdentifiedDeviceDriver {
    async fn process(&mut self) -> Result<(), DriverProcessError> {
        if let Some(driver) = &mut self.driver {
            driver.process().await
        } else {
            // Not all drivers are required
            Ok(())
        }
    }
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