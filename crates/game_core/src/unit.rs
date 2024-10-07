pub mod pinentry;

use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait GameCoreUnit {
    async fn process(&mut self) -> Result<()>;
}
