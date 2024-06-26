use std::time::Duration;

pub mod headset_gyroscope;

#[derive(Debug)]
pub enum DriverProcessError {
    DataframeError(String),
    IoError,
}

pub trait DeviceDriver {
    fn process(&mut self) -> Result<(), DriverProcessError>;
    fn last_communication(&self) -> Duration;
}