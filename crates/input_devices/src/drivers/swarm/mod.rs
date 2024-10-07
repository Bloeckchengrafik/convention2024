pub(crate) mod steering_wheel;

use ftswarm::prelude::*;
use ftswarm_serial::SerialCommunication;

aliases! {
    FtSwarmAliases {
        BUTTON_1 = "ftSwarm133.A1",
        BUTTON_2 = "ftSwarm133.A2",
        WHEEL = "ftSwarm133.A3",
    }
}

#[derive(Clone)]
pub struct VrSwarm {
    pub button_1: Io<Switch>,
    pub button_2: Io<Switch>,
    pub wheel: Io<RotaryEncoder>,

    pub _swarm: FtSwarm,
}

impl VrSwarm {
    pub(crate) async fn new(value: &str) -> Self {
        let swarm = FtSwarm::new(SerialCommunication::connect(&value));
        VrSwarm {
            button_1: Switch::create(&swarm, FtSwarmAliases::BUTTON_1, NormallyOpen::Closed).await,
            button_2: Switch::create(&swarm, FtSwarmAliases::BUTTON_2, NormallyOpen::Closed).await,
            wheel: RotaryEncoder::create(&swarm, FtSwarmAliases::WHEEL, true).await,
            _swarm: swarm,
        }
    }
}
