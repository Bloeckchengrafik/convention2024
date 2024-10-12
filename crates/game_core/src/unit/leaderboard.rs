use std::time::Instant;
use async_trait::async_trait;
use pub_sub::{PubSub, Subscription};
use messages::{LeaderboardEntry, VrMessage};
use messages::file_config::{read_config, save_config};
use crate::unit::GameCoreUnit;

struct LeaderboardUser {
    name: String,
    start: Instant,
}

impl Into<LeaderboardEntry> for LeaderboardUser {
    fn into(self) -> LeaderboardEntry {
        let entries = read_config().leaderboard;
        let next_id = entries.iter().map(|entry| entry.id).max().unwrap_or(0) + 1;
        LeaderboardEntry {
            name: self.name,
            time: self.start.elapsed().as_secs_f32(),
            id: next_id,
        }
    }
}

pub struct Leaderboard {
    bus: PubSub<VrMessage>,
    subscription: Subscription<VrMessage>,
    start: Option<LeaderboardUser>,
}

impl Leaderboard {
    pub fn new(bus: &PubSub<VrMessage>) -> Self {
        let subscription = bus.subscribe();
        Self {
            bus: bus.clone(),
            subscription,
            start: None,
        }
    }
}


#[async_trait]
impl GameCoreUnit for Leaderboard {
    async fn process(&mut self) -> anyhow::Result<()> {
        while let Ok(message) = self.subscription.try_recv() {
            match message {
                VrMessage::TimerStart { name } => {
                    self.start = Some(LeaderboardUser {
                        name,
                        start: Instant::now(),
                    });
                }
                VrMessage::TimerEnd {} => {
                    if let Some(user) = self.start.take() {
                        let entry: LeaderboardEntry = user.into();

                        let mut config = read_config();
                        config.leaderboard.push(entry.clone());
                        save_config(&config);

                        let _ = self.bus.send(VrMessage::PushTimerEntry { entry });
                    }
                }
                VrMessage::DeleteTimerEntry { id } => {
                    let mut config = read_config();
                    config.leaderboard.retain(|entry| entry.id != id);
                    save_config(&config);
                }
                _ => {}
            }
        }
        Ok(())
    }
}