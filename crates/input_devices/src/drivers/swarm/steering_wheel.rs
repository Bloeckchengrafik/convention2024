use async_trait::async_trait;
use pub_sub::{PubSub, Subscription};
use messages::VrMessage;
use crate::drivers::{DeviceDriver, DriverProcessError};
use crate::drivers::swarm::VrSwarm;

pub struct SteeringWheelDriver {
    swarm: VrSwarm,
    bus: PubSub<VrMessage>,
    subscription: Subscription<VrMessage>,
    rotation: i128,
    flipped_buttons: bool,
    button_1: bool,
    button_2: bool,
    offset: i128,
}

impl SteeringWheelDriver {
    pub fn new(value: &VrSwarm, bus: &PubSub<VrMessage>) -> Box<Self> {
        Box::new(SteeringWheelDriver {
            swarm: value.clone(),
            bus: bus.clone(),
            subscription: bus.subscribe(),
            rotation: 0,
            flipped_buttons: false,
            button_1: false,
            button_2: false,
            offset: 0,
        })
    }

    fn as_state_transfer(&self) -> VrMessage {
        VrMessage::WheelState {
            rotation: self.rotation - self.offset,
            left_button: if self.flipped_buttons { self.button_2 } else { self.button_1 },
            right_button: if self.flipped_buttons { self.button_1 } else { self.button_2 },
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

        self.rotation = -self.swarm.wheel.lock().unwrap().value as i128;
        self.button_1 = self.swarm.button_1.lock().unwrap().value;
        self.button_2 = self.swarm.button_2.lock().unwrap().value;

        self.bus.send(self.as_state_transfer()).map_err(|_| DriverProcessError::BusError)?;
        Ok(())
    }
}