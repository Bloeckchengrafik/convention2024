use async_trait::async_trait;
use ftswarm::prelude::{Io, SwarmObject, Ohmmeter, Hysteresis};
use pub_sub::{PubSub, Subscription};
use messages::{PedalPosition, VrMessage};
use messages::file_config::{read_config, save_config};
use crate::drivers::{DeviceDriver, DriverProcessError};
use crate::drivers::swarm::{FtSwarmAliases, VrSwarm};

pub struct PedalDriver {
    swarm: VrSwarm,
    bus: PubSub<VrMessage>,
    subscription: Subscription<VrMessage>,
    lidar: Io<Ohmmeter>,
    min: i32,
    max: i32,
    last_n: [i32; 5],
    idx: usize,
    last_read: std::time::Instant,
    current_minmax: u8,
}

impl PedalDriver {
    pub async fn new(swarm: VrSwarm, bus: PubSub<VrMessage>) -> Box<dyn DeviceDriver> {
        let conf = read_config();
        Box::new(PedalDriver {
            subscription: bus.subscribe(),
            bus,
            min: conf.pedal_calibration_lower,
            max: conf.pedal_calibration_upper,
            last_n: [0; 5],
            idx: 0,
            lidar: Ohmmeter::create(&swarm.lib, FtSwarmAliases::THROTTLE, Hysteresis(0)).await,
            swarm,
            last_read: std::time::Instant::now(),
            current_minmax: 0,
        })
    }

    fn as_state_transfer(&self) -> VrMessage {
        VrMessage::PedalState {
            pressed: self.current_minmax
        }
    }
}

#[async_trait]
impl DeviceDriver for PedalDriver {
    async fn process(&mut self) -> Result<(), DriverProcessError> {
        while let Ok(message) = self.subscription.try_recv() {
            match message {
                VrMessage::ZeroPedal { position } => {
                    match position {
                        PedalPosition::Lower => {
                            self.min = self.last_n.iter().sum::<i32>() / 5;
                            let mut conf = read_config();
                            conf.pedal_calibration_lower = self.min;
                            save_config(&conf);
                        }
                        PedalPosition::Upper => {
                            self.max = self.last_n.iter().sum::<i32>() / 5;
                            let mut conf = read_config();
                            conf.pedal_calibration_upper = self.max;
                            save_config(&conf);
                        }
                    }
                }
                _ => {}
            }
        }


        if self.last_read.elapsed().as_millis() < 100 {
            return Ok(());
        }

        self.last_read = std::time::Instant::now();
        self.last_n[self.idx] = self.lidar.lock().await.value;
        self.idx = (self.idx + 1) % 5;
        if self.min == 0 {
            self.min = 1;
        }

        if self.max == 0 {
            self.max = 1;
        }

        if self.max == self.min {
            self.max += 1;
        }

        let mut current = self.last_n.iter().sum::<i32>() / 5;
        if current < self.min {
            current = self.min;
        } else if current > self.max {
            current = self.max;
        }

        self.current_minmax = ((current - self.min) as f32 / (self.max - self.min) as f32 * 100.0) as u8;

        if self.current_minmax < 20 {
            self.current_minmax = 0;
        }


        self.bus.send(self.as_state_transfer()).map_err(|_| DriverProcessError::BusError)?;
        Ok(())
    }
}