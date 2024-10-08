use async_trait::async_trait;
use ftswarm::prelude::{Io, NormallyOpen, RotaryEncoder, SwarmObject, Switch};
use pub_sub::{PubSub, Subscription};
use messages::VrMessage;
use crate::drivers::{DeviceDriver, DriverProcessError};
use crate::drivers::swarm::{FtSwarmAliases, VrSwarm};

pub struct SteeringWheelDriver {
    swarm: VrSwarm,
    bus: PubSub<VrMessage>,
    subscription: Subscription<VrMessage>,
    rotation: i128,
    flipped_buttons: bool,
    button_1_val: bool,
    button_2_val: bool,
    offset: i128,
    pub button_1: Io<Switch>,
    pub button_2: Io<Switch>,
    pub wheel: Io<RotaryEncoder>,
}

impl SteeringWheelDriver {
    pub async fn new(swarm: VrSwarm, bus: PubSub<VrMessage>) -> Box<dyn DeviceDriver> {
        Box::new(SteeringWheelDriver {
            subscription: bus.subscribe(),
            bus,
            rotation: 0,
            flipped_buttons: false,
            button_1_val: false,
            button_2_val: false,
            offset: 0,
            button_1: Switch::create(&swarm.lib, FtSwarmAliases::BUTTON_1, NormallyOpen::Closed).await,
            button_2: Switch::create(&swarm.lib, FtSwarmAliases::BUTTON_2, NormallyOpen::Closed).await,
            wheel: RotaryEncoder::create(&swarm.lib, FtSwarmAliases::WHEEL, true).await,
            swarm,
        })
    }

    fn as_state_transfer(&self) -> VrMessage {
        VrMessage::WheelState {
            rotation: self.rotation - self.offset,
            left_button: if self.flipped_buttons { self.button_2_val } else { self.button_1_val },
            right_button: if self.flipped_buttons { self.button_1_val } else { self.button_2_val },
            flipped: self.flipped_buttons,
        }
    }
}

#[async_trait]
impl DeviceDriver for SteeringWheelDriver {
    async fn process(&mut self) -> Result<(), DriverProcessError> {
        while let Ok(message) = self.subscription.try_recv() {
            match message {
                VrMessage::FlipWheelBtns { flip } => self.flipped_buttons = flip,
                VrMessage::ResetWheel {} => self.offset = self.rotation,
                _ => {}
            }
        }

        self.rotation = -self.wheel.lock().await.value as i128;
        self.button_1_val = self.button_1.lock().await.value;
        self.button_2_val = self.button_2.lock().await.value;

        self.bus.send(self.as_state_transfer()).map_err(|_| DriverProcessError::BusError)?;
        Ok(())
    }
}