use pub_sub::PubSub;
use tokio::task::JoinSet;
use messages::LogMessageType::Error;
use messages::VrMessage;
use unit::GameCoreUnit;

pub mod unit;
mod api;

struct Units {
    units: Vec<Box<dyn GameCoreUnit + Send>>,
}

impl Units {
    fn from_bus(bus: &PubSub<VrMessage>) -> Self {
        Self {
            units: vec![
                Box::new(unit::pinentry::PinEntry::new(&bus)),
            ],
        }
    }

    fn finish(self) -> Vec<Box<dyn GameCoreUnit + Send>> {
        self.units
    }
}

async fn unit_loop(mut unit: Box<dyn GameCoreUnit + Send>, bus: PubSub<VrMessage>) {
    loop {
        let result = unit.process().await;
        if let Err(e) = result {
            let _ = bus.send(VrMessage::Log {
                message: format!("Error: {}", e),
                message_type: Error,
            });
        }
    }
}

pub async fn game_main(bus: PubSub<VrMessage>) {
    let mut units = Units::from_bus(&bus).finish();
    let mut join_set = JoinSet::new();

    while let Some(unit) = units.pop() {
        join_set.spawn(unit_loop(unit, bus.clone()));
    }

    join_set.join_all().await;
}