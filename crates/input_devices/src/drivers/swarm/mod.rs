pub(crate) mod steering_wheel;
pub(crate) mod car;
pub(crate) mod pedal;

use ftswarm::prelude::*;
use ftswarm_serial::SerialCommunication;

aliases! {
    FtSwarmAliases {
        BUTTON_1 = "A1",
        BUTTON_2 = "A2",
        WHEEL = "A3",
        THROTTLE = "A5",
        CAR_STEER = "ftSwarm106.SERVO1",
        CAR_CAM_PITCH = "ftSwarm106.SERVO2",
        CAR_CAM_YAW = "ftSwarm106.SERVO3",
        CAR_THROTTLE = "ftSwarm106.M2",
    }
}

#[derive(Clone)]
pub struct VrSwarm {
    pub lib: FtSwarm,
}

impl VrSwarm {
    pub(crate) async fn new(value: &str) -> Self {
        let swarm = FtSwarm::new(SerialCommunication::connect(&value));
        VrSwarm {
            lib: swarm,
        }
    }
}
