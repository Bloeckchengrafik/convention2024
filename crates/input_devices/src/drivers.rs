use std::time::Duration;

pub mod headset_gyroscope;

pub enum DriverDiscoveryError {
    SignatureMismatch,
}

#[derive(Debug)]
pub enum DriverProcessError {
    DataframeError(String),
}

pub trait DeviceDriver {
    fn process(&mut self) -> Result<(), DriverProcessError>;
    fn last_communication(&self) -> Duration;
}