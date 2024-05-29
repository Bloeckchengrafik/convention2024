use crate::drivers::{DeviceDriver, DriverProcessError};

pub mod autodetect;
mod drivers;

pub struct InputDevices {
    pub headset_gyroscope: drivers::headset_gyroscope::HeadsetGyroscopeDeviceDriver,
}

impl InputDevices {
    pub fn new() -> InputDevices {
        return autodetect::autodetect_input_devices();
    }

    pub fn process(&mut self) -> Result<(), DriverProcessError> {
        self.headset_gyroscope.process()?;

        return Ok(());
    }
}