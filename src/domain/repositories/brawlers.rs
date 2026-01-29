use anyhow::Result;
use async_trait::async_trait;
use mockall::automock;
use crate::domain::entities::{brawlers::{BrawlerEntity, RegisterBrawlerEntity}, missions::MissionEntity};

#[async_trait]
#[automock]
pub trait BrawlerRepository {
    async fn register(&self, register_brawler_entity: RegisterBrawlerEntity) -> Result<i32>;
    async fn find_by_username(&self, username: String) -> Result<BrawlerEntity>;
    async fn update_avatar(&self, id: i32, avatar_url: String) -> Result<String>;
    async fn crew_counting(&self, mission_id: i32) -> Result<u32>;
    async fn get_missions(&self, brawler_id: i32) -> Result<Vec<MissionEntity>>;
}
