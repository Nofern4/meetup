use std::sync::Arc;
use anyhow::Result;
use async_trait::async_trait;
use diesel::{ExpressionMethods, PgTextExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper};

use crate::{domain::{entities::missions::MissionEntity, repositories::mission_viewing::MissionViewingRepository, value_objects::mission_filter::MissionFilter}, infrastructure::database::{postgresql_connection::PgPoolSquad, schema::{crew_memberships, missions}}};

pub struct MisssionViewingPostgres {
    db_pool: Arc<PgPoolSquad>,
}

impl MisssionViewingPostgres {
    pub fn new(db_pool: Arc<PgPoolSquad>) -> Self {
        Self { db_pool }
    }
}

#[async_trait]
impl MissionViewingRepository for MisssionViewingPostgres {
   async fn view_detail(&self, mission_id: i32) -> Result<MissionEntity> {
        let mut conn = Arc::clone(&self.db_pool).get()?;

        let result = missions::table
            .filter(missions::id.eq(mission_id))
            .filter(missions::delete_at.is_null())
            .select(MissionEntity::as_select())
            .first::<MissionEntity>(&mut conn)?;

        Ok(result)
    }

    async fn get(&self, filter: &MissionFilter) -> Result<Vec<MissionEntity>> {
        let mut conn = Arc::clone(&self.db_pool).get()?;

        let mut query = missions::table
            .filter(missions::delete_at.is_null())
            .into_boxed();

        if let Some(status) = &filter.status {
            query = query.filter(missions::status.eq(status.to_string()));
        };

        if let Some(name) = &filter.name {
            query = query.filter(missions::name.ilike(format!("%{}%", name)));
        }

        let results = query
            .select(MissionEntity::as_select())
            .order_by(missions::create_at.desc())
            .load::<MissionEntity>(&mut conn)?;

        Ok(results)
    }

    async fn crew_counting(&self, mission_id: i32) -> Result<u32> {
        let mut conn = Arc::clone(&self.db_pool).get()?;

        let result = crew_memberships::table
            .filter(crew_memberships::mission_id.eq(mission_id))
            .count()
            .first::<i64>(&mut conn)?;

        let count = u32::try_from(result)?;

        Ok(count)
    }

}