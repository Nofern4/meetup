use anyhow::Result;
use async_trait::async_trait;
use mockall::automock;

use crate::domain::{entities::missions::MissionEntity, value_objects::mission_filter::MissionFilter};

#[async_trait]
#[automock]
pub trait MissionViewingRepository {
    async fn view_detail(&self, mission_id: i32) -> Result<MissionEntity>;
    async fn get(&self, filter: &MissionFilter) -> Result<Vec<MissionEntity>>;
    async fn crew_counting(&self, mission_id: i32) -> Result<u32>;
}
