use async_trait::async_trait;
use shared::GameMetadata;
use uuid::Uuid;

#[async_trait]
pub trait GameManager: Send + Sync {
    async fn create_game(&self, game_name: String) -> anyhow::Result<GameMetadata>;
    async fn list_games(&self) -> anyhow::Result<Vec<GameMetadata>>;
    async fn delete_game(&self, game_id: Uuid) -> anyhow::Result<()>;
}
