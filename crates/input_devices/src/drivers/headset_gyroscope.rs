use std::time::Duration;
use ftswarm_serial::{SerialCommunication, SwarmSerialPort};
use crate::drivers::{DeviceDriver, DriverDiscoveryError, DriverProcessError};
use crate::drivers::headset_gyroscope::GyroscopeDataframeError::InvalidDataframe;

#[derive(Debug, Default)]
pub struct GyroscopeDataframe {
    pub yaw: f32,
    pub pitch: f32,
    pub roll: f32,

    pub delta_yaw: f32,
    pub delta_pitch: f32,
    pub delta_roll: f32,

    pub temperature: f32,
}

pub enum GyroscopeDataframeError {
    InvalidDataframe,
}

impl TryFrom<String> for GyroscopeDataframe {
    type Error = GyroscopeDataframeError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let value = value.trim_matches(|c| c == '\n' || c == '\r');
        // value has to start with 0x
        if !value.starts_with("0x") {
            return Err(InvalidDataframe);
        }

        let value = &value[2..];

        // Split the string into parts (:) and convert them to floats
        let parts: Vec<&str> = value.split(':')
            .map(|x| x.trim())
            .collect();

        if parts.len() != 7 {
            return Err(InvalidDataframe);
        }

        let (
            yaw,
            pitch,
            roll,
            delta_yaw,
            delta_pitch,
            delta_roll,
            temperature,
        ) = (
            parts[0].parse::<f32>().unwrap(),
            parts[1].parse::<f32>().unwrap(),
            parts[2].parse::<f32>().unwrap(),
            parts[3].parse::<f32>().unwrap(),
            parts[4].parse::<f32>().unwrap(),
            parts[5].parse::<f32>().unwrap(),
            parts[6].parse::<f32>().unwrap(),
        );

        Ok(GyroscopeDataframe {
            yaw,
            pitch,
            roll,
            delta_yaw,
            delta_pitch,
            delta_roll,
            temperature,
        })
    }
}

pub struct HeadsetGyroscopeDeviceDriver {
    port: SerialCommunication,
    pub last_data: GyroscopeDataframe,
    last_timestamp: std::time::Instant,
}

impl HeadsetGyroscopeDeviceDriver {
    pub fn new(port: SerialCommunication) -> Self {
        HeadsetGyroscopeDeviceDriver {
            port,
            last_data: GyroscopeDataframe::default(),
            last_timestamp: std::time::Instant::now(),
        }
    }
    pub fn check_discovery_signature(port: &mut SerialCommunication) -> bool {
        while port.available() {
            port.read_line();
        }

        port.write_line("whoami".to_string());
        std::thread::sleep(Duration::from_millis(500));
        let response = port.read_line();
        if response.starts_with("0x") {
            return true;
        }

        if !response.starts_with("raw/accelerometer") {
            return false;
        }

        return true;
    }

    fn process_available_line(&mut self, line: String) -> Result<(), GyroscopeDataframeError> {
        let data = GyroscopeDataframe::try_from(line)?;

        self.last_data = data;
        self.last_timestamp = std::time::Instant::now();

        Ok(())
    }
}

impl DeviceDriver for HeadsetGyroscopeDeviceDriver {
    fn process(&mut self) -> Result<(), DriverProcessError> {
        while self.port.available() {
            let line = self.port.read_line();
            self.process_available_line(line).map_err(|_| DriverProcessError::DataframeError("Invalid Gyroscope Dataframe".into()))?;
        }

        Ok(())
    }

    fn last_communication(&self) -> Duration {
        self.last_timestamp.elapsed()
    }
}