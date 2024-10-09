use async_trait::async_trait;
use ftswarm::prelude::{Io, Motor, Servo, SwarmObject};
use pub_sub::{PubSub, Subscription};
use messages::file_config::{read_config, save_config};
use messages::VrMessage;
use crate::drivers::{DeviceDriver, DriverProcessError};
use crate::drivers::swarm::{FtSwarmAliases, VrSwarm};

pub struct CarDriver {
    pub(crate) swarm: VrSwarm,
    subscription: Subscription<VrMessage>,
    last_5_yaws: [f32; 5],
    last_5_pitches: [f32; 5],
    wheel_pos: i128,
    idx: usize,
    steering_servo: Io<Servo>,
    throttle_motor: Io<Motor>,
    cam_yaw_servo: Io<Servo>,
    cam_pitch_servo: Io<Servo>,
    old_offset_steer: i32,
    old_offset_cam_yaw: i32,
    old_offset_cam_pitch: i32,
    interface_open: bool,
    last_write: std::time::Instant,
}

impl CarDriver {
    pub async fn new(swarm: VrSwarm, bus: PubSub<VrMessage>) -> Box<dyn DeviceDriver> {
        Box::new(CarDriver {
            subscription: bus.subscribe(),
            last_5_yaws: [0.0; 5],
            last_5_pitches: [0.0; 5],
            wheel_pos: 0,
            idx: 0,
            steering_servo: Servo::create(&swarm.lib, FtSwarmAliases::CAR_STEER, ()).await,
            throttle_motor: Motor::create(&swarm.lib, FtSwarmAliases::CAR_THROTTLE, ()).await,
            cam_yaw_servo: Servo::create(&swarm.lib, FtSwarmAliases::CAR_CAM_YAW, ()).await,
            cam_pitch_servo: Servo::create(&swarm.lib, FtSwarmAliases::CAR_CAM_PITCH, ()).await,
            old_offset_steer: 0,
            old_offset_cam_yaw: 0,
            old_offset_cam_pitch: 0,
            interface_open: false,
            last_write: std::time::Instant::now(),
            swarm,
        })
    }

    fn clamp(value: f32, min: f32, max: f32) -> f32 {
        if value < min {
            min
        } else if value > max {
            max
        } else {
            value
        }
    }

    fn remap(value: f32, from_min: f32, from_max: f32, to_min: f32, to_max: f32) -> f32 {
        (value - from_min) / (from_max - from_min) * (to_max - to_min) + to_min
    }
}

#[async_trait]
impl DeviceDriver for CarDriver {
    async fn process(&mut self) -> Result<(), DriverProcessError> {
        while let Ok(message) = self.subscription.try_recv() {
            match message {
                VrMessage::GyroscopeReading { yaw, pitch, .. } => {
                    self.last_5_yaws[self.idx] = yaw;
                    self.last_5_pitches[self.idx] = pitch;
                    self.idx = (self.idx + 1) % 5;

                    if self.interface_open {
                        continue;
                    }

                    let avg_yaw = self.last_5_yaws.iter().sum::<f32>() / 5.0;
                    let avg_pitch = self.last_5_pitches.iter().sum::<f32>() / 5.0;
                    let mapped_steer = Self::remap(avg_yaw, -200.0, 200.0, -90.0, 90.0) as i32;

                    let final_yaw = Self::clamp(avg_yaw, -90.0, 90.0) as i32;
                    let final_pitch = Self::clamp(avg_pitch, -90.0, 90.0) as i32;

                    let now = std::time::Instant::now();
                    if now.duration_since(self.last_write).as_millis() > 100 {
                        self.steering_servo.lock().await.set_position(mapped_steer).await.map_err(|_| DriverProcessError::SwarmError)?;
                        self.cam_yaw_servo.lock().await.set_position(final_yaw).await.map_err(|_| DriverProcessError::SwarmError)?;
                        self.cam_pitch_servo.lock().await.set_position(final_pitch).await.map_err(|_| DriverProcessError::SwarmError)?;
                        self.last_write = now;
                    }

                }
                VrMessage::ShowRenderedInterface { .. } => {
                    self.interface_open = true;
                }
                VrMessage::InterfaceConfirm { .. } => {
                    self.interface_open = false;
                }
                VrMessage::WheelState { rotation, .. } => {
                    self.wheel_pos = rotation;
                }
                VrMessage::SetServoConfig { config } => {
                    let mut fileconfig = read_config();
                    fileconfig.servo_config = config;
                    save_config(&fileconfig);
                }
                _ => {}
            }
        }

        let config = read_config();
        let new_offset_steer = config.servo_config.steer_offset;
        let new_offset_cam_yaw = config.servo_config.yaw_offset;
        let new_offset_cam_pitch = config.servo_config.pitch_offset;
        {
            if new_offset_steer != self.old_offset_steer {
                self.old_offset_steer = new_offset_steer;
                self.steering_servo.lock().await.set_offset(new_offset_steer).await.map_err(|_| DriverProcessError::SwarmError)?;
            }

            if new_offset_cam_yaw != self.old_offset_cam_yaw {
                self.old_offset_cam_yaw = new_offset_cam_yaw;
                self.cam_yaw_servo.lock().await.set_offset(new_offset_cam_yaw).await.map_err(|_| DriverProcessError::SwarmError)?;
            }

            if new_offset_cam_pitch != self.old_offset_cam_pitch {
                self.old_offset_cam_pitch = new_offset_cam_pitch;
                self.cam_pitch_servo.lock().await.set_offset(new_offset_cam_pitch).await.map_err(|_| DriverProcessError::SwarmError)?;
            }
        }
        Ok(())
    }
}