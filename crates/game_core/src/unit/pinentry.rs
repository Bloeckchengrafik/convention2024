use async_trait::async_trait;
use pub_sub::{PubSub, Subscription};
use messages::{Interface, VrMessage};
use anyhow::{bail, Result};
use crate::api::AsyncBus;
use crate::unit::GameCoreUnit;

pub struct PinEntry {
    bus: PubSub<VrMessage>,
    subscription: Subscription<VrMessage>,
    async_bus: AsyncBus,
}

impl PinEntry {
    pub fn new(bus: &PubSub<VrMessage>) -> Self {
        let subscription = bus.subscribe();
        Self {
            bus: bus.clone(),
            subscription,
            async_bus: AsyncBus::new(&bus),
        }
    }

    async fn pin_entry(&mut self, pin_len: u8) -> Result<String> {
        let mut pin = String::new();
        for _ in 0..pin_len {
            let data = self.async_bus.open_interface_and_confirm(Interface::InputNumberAndConfirm {
                text: format!("PIN: {}", pin),
            }).await?;

            pin.push_str(&data.to_string());
        }

        Ok(pin)
    }
}

#[async_trait]
impl GameCoreUnit for PinEntry {
    async fn process(&mut self) -> Result<()> {
        if let Ok(message) = self.subscription.try_recv() {
            match message {
                VrMessage::AskPin { length } => {
                    let pin = self.pin_entry(length).await?;
                    self.bus.send(VrMessage::ConfirmPin { pin })?;
                }
                _ => {}
            }
        }
        Ok(())
    }
}