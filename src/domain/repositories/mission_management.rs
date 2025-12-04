use anyhow::Result;
use async_trait::async_trait;
use mockall::automock;

use crate::domain::entities::missions::{NewMission, UpdateMission};

#[async_trait]
#[automock]
pub trait MissionManagementRepository {
    async fn add(&self, add_mission_entity: NewMission) -> Result<i32>;
    async fn edit(&self, mission_id: i32, edit_mission_entity: UpdateMission) -> Result<i32>;
    async fn remove(&self, mission_id: i32, chief_id: i32) -> Result<()>;
}
