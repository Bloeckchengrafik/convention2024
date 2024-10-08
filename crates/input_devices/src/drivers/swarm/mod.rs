pub(crate) mod steering_wheel;
pub(crate) mod car;

use ftswarm::prelude::*;
use ftswarm_serial::SerialCommunication;

aliases! {
    FtSwarmAliases {
        BUTTON_1 = "ftSwarm133.A1",
        BUTTON_2 = "ftSwarm133.A2",
        WHEEL = "ftSwarm133.A3",
        CAR_STEER = "Car.SERVO1",
        CAR_CAM_YAW = "Car.SERVO2",
        CAR_CAM_PITCH = "Car.SERVO3",
        CAR_THROTTLE = "Car.M1",
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
