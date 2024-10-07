use anyhow::bail;
use pub_sub::{PubSub, Subscription};
use messages::{Interface, VrMessage};

pub struct AsyncBus {
    publication: PubSub<VrMessage>,
    subscription: Subscription<VrMessage>,
}

impl AsyncBus {
    pub fn new(pub_sub: &PubSub<VrMessage>) -> Self {
        Self {
            publication: pub_sub.clone(),
            subscription: pub_sub.subscribe(),
        }
    }

    async fn wait_on_message(&mut self, message_filter: fn(&VrMessage) -> bool) -> VrMessage {
        loop {
            if let Ok(message) = self.subscription.try_recv() {
                if message_filter(&message) {
                    return message;
                }
            }

            tokio::task::yield_now().await;
        }
    }

    pub async fn open_interface(&mut self, interface: Interface) -> anyhow::Result<()> {
        self.publication.send(VrMessage::ShowRenderedInterface { interface })?;
        Ok(())
    }

    pub async fn confirm_interface(&mut self) -> anyhow::Result<Interface> {
        match self.wait_on_message(|m| matches!(m, VrMessage::InterfaceConfirm {..})).await {
            VrMessage::InterfaceConfirm { data } => Ok(data),
            _ => bail!("Unexpected message"),
        }
    }

    pub async fn open_interface_and_confirm(&mut self, interface: Interface) -> anyhow::Result<Interface> {
        self.open_interface(interface).await?;
        self.confirm_interface().await
    }
}