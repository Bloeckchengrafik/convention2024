use std::io::Read;
use std::ops::Sub;
use std::time::Duration;
use serialport::SerialPort;
use crate::drivers::{DeviceDriver, DriverProcessError};
use crate::drivers::headset_gyroscope::GyroscopeDataframeError::{LenInvalid, NumFormat, StartInvalid};

#[derive(Debug, Default, Copy, Clone)]
pub struct GyroscopeDataframe {
    pub yaw: f32,
    pub pitch: f32,
    pub roll: f32,
}

impl Sub for GyroscopeDataframe {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        GyroscopeDataframe {
            yaw: self.yaw - rhs.yaw,
            pitch: self.pitch - rhs.pitch,
            roll: self.roll - rhs.roll,
        }
    }
}

#[derive(Debug)]
pub enum GyroscopeDataframeError {
    StartInvalid,
    LenInvalid {
        expected: usize,
        got: usize,
    },
    NumFormat(std::num::ParseFloatError),
}

impl GyroscopeDataframe {
    fn try_from(value: String, driver: &mut HeadsetGyroscopeDeviceDriver) -> Result<Self, GyroscopeDataframeError> {
        let value = value.trim_matches(|c| c == '\n' || c == '\r');
        // value has to start with 0x
        if !value.starts_with("0x") {
            return Err(StartInvalid);
        }

        let value = &value[2..];

        // Split the string into parts (:) and convert them to floats
        let parts: Vec<&str> = value.split(':')
            .map(|x| x.trim())
            .collect();

        if parts.len() != 3 {
            return Err(LenInvalid {
                expected: 3,
                got: parts.len(),
            });
        }

        let (
            yaw_deg,
            pitch_deg,
            roll_deg,
        ) = (
            parts[0].parse::<f32>().map_err(|e| NumFormat(e))?,
            parts[1].parse::<f32>().map_err(|e| NumFormat(e))?,
            parts[2].parse::<f32>().map_err(|e| NumFormat(e))?,
        );

        let (
            yaw,
            pitch,
            roll,
        ) = (
            yaw_deg.to_radians(),
            pitch_deg.to_radians(),
            roll_deg.to_radians(),
        );

        driver.last_raw_data = GyroscopeDataframe {
            yaw,
            pitch,
            roll,
        };

        Ok(GyroscopeDataframe {
            yaw,
            pitch,
            roll,
        } - driver.zero_offset.unwrap_or(GyroscopeDataframe::default()))
    }
}

pub struct HeadsetGyroscopeDeviceDriver {
    port: Box<dyn SerialPort>,
    pub last_data: GyroscopeDataframe,
    last_raw_data: GyroscopeDataframe,
    last_timestamp: std::time::Instant,
    line_buffer: String,
    zero_offset: Option<GyroscopeDataframe>,
}

impl HeadsetGyroscopeDeviceDriver {
    pub fn new(port: Box<dyn SerialPort>) -> Self {
        HeadsetGyroscopeDeviceDriver {
            port,
            last_data: GyroscopeDataframe::default(),
            last_raw_data: GyroscopeDataframe::default(),
            last_timestamp: std::time::Instant::now(),
            line_buffer: String::new(),
            zero_offset: None,
        }
    }
    pub fn check_discovery_signature(port: &mut Box<dyn SerialPort>) -> bool {
        Self::read_buffer_full(port);

        std::thread::sleep(std::time::Duration::from_millis(700));

        let mut response = vec![0; 2];
        port.read_exact(&mut response).unwrap();
        if response.starts_with(b"0x") {
            return true;
        }

        Self::read_buffer_full(port);

        return true;
    }

    pub fn zero(&mut self) {
        self.zero_offset = Some(self.last_raw_data);
    }

    fn read_buffer_full(port: &mut Box<dyn SerialPort>) {
        if let Ok(t) = port.bytes_to_read() {
            if t > 0 {
                let mut buf = vec![0; t as usize];
                port.read_exact(&mut buf).unwrap();
            }
        }
    }

    fn process_available_line(&mut self, line: String) -> Result<(), GyroscopeDataframeError> {
        let data = GyroscopeDataframe::try_from(line, self)?;

        self.last_data = data;
        self.last_timestamp = std::time::Instant::now();

        Ok(())
    }

    fn read_one_line(&mut self) -> Option<String> {
        let mut line = self.line_buffer.clone();
        self.line_buffer.clear();
        loop {
            let mut buf = [0; 1];
            if self.port.bytes_to_read().unwrap() == 0 {
                self.line_buffer = line;
                return None;
            }

            let result = self.port.read_exact(&mut buf);
            if result.is_err() {
                self.line_buffer = line;
                return None;
            }
            let c = buf[0] as char;
            if c == '\n' {
                break;
            }
            line.push(c);
        }

        Some(line)
    }
}

impl DeviceDriver for HeadsetGyroscopeDeviceDriver {
    fn process(&mut self) -> Result<(), DriverProcessError> {
        loop {
            let line = self.read_one_line();
            if let Some(line) = line {
                self.process_available_line(line.clone()).map_err(|e| DriverProcessError::DataframeError(format!("Invalid dataframe: {:?} ({})", e, line)))?;
            } else {
                break;
            }
        }

        Ok(())
    }

    fn last_communication(&self) -> Duration {
        self.last_timestamp.elapsed()
    }
}