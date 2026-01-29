use anyhow::Result;
use async_trait::async_trait;
use diesel::{ExpressionMethods, RunQueryDsl};
use std::sync::Arc;

use crate::{
    domain::{
        repositories::mission_operation::MissionOperationRepository,
        value_objects::mission_statuses::MissionStatuses,
    },
    infrastructure::database::{postgresql_connection::PgPoolSquad, schema::missions},
};

pub struct MissionOperationPostgres {
    db_pool: Arc<PgPoolSquad>,
}

impl MissionOperationPostgres {
    pub fn new(db_pool: Arc<PgPoolSquad>) -> Self {
        Self { db_pool }
    }
}

#[async_trait]
impl MissionOperationRepository for MissionOperationPostgres {
    async fn in_progress(&self, mission_id: i32, chief_id: i32) -> Result<i32> {
        let mut connection = Arc::clone(&self.db_pool).get()?;

        let result = diesel::update(missions::table)
            .filter(missions::id.eq(mission_id))
            .filter(missions::chief_id.eq(chief_id))
            .set(missions::status.eq(MissionStatuses::InProgress.to_string()))
            .returning(missions::id)
            .get_result::<i32>(&mut connection)?;

        Ok(result)
    }

    async fn to_completed(&self, mission_id: i32, chief_id: i32) -> Result<i32> {
        let mut connection = Arc::clone(&self.db_pool).get()?;

        let result = diesel::update(missions::table)
            .filter(missions::id.eq(mission_id))
            .filter(missions::chief_id.eq(chief_id))
            .set(missions::status.eq(MissionStatuses::Completed.to_string()))
            .returning(missions::id)
            .get_result::<i32>(&mut connection)?;

        Ok(result)
    }

    async fn to_failed(&self, mission_id: i32, chief_id: i32) -> Result<i32> {
        let mut connection = Arc::clone(&self.db_pool).get()?;

        let result = diesel::update(missions::table)
            .filter(missions::id.eq(mission_id))
            .filter(missions::chief_id.eq(chief_id))
            .set(missions::status.eq(MissionStatuses::Failed.to_string()))
            .returning(missions::id)
            .get_result::<i32>(&mut connection)?;

        Ok(result)
    }
}
