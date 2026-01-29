use std::sync::Arc;
use anyhow::Result;
use async_trait::async_trait;
use diesel::{ExpressionMethods, RunQueryDsl, dsl::insert_into, QueryDsl, SelectableHelper, BoolExpressionMethods};
use crate::infrastructure::database::{postgresql_connection::PgPoolSquad, schema::{brawlers, crew_memberships, missions}};
use crate::domain::{entities::{brawlers::{BrawlerEntity, RegisterBrawlerEntity}, missions::MissionEntity}, repositories::brawlers::BrawlerRepository};

pub struct BrawlerPostgres {
    db_pool: Arc<PgPoolSquad>,
}

impl BrawlerPostgres {
    pub fn new(db_pool: Arc<PgPoolSquad>) -> Self {
        Self { db_pool }
    }
}

#[async_trait]
impl BrawlerRepository for BrawlerPostgres {
    async fn register(&self, register_brawler_entity: RegisterBrawlerEntity) -> Result<i32> {
        let mut connection = Arc::clone(&self.db_pool).get()?;

        let result = insert_into(brawlers::table)
            .values(&register_brawler_entity)
            .returning(brawlers::id)
            .get_result::<i32>(&mut connection)?;

        Ok(result)
    }

    async fn find_by_username(&self, username: String) -> Result<BrawlerEntity> {
        let mut connection = Arc::clone(&self.db_pool).get()?;

        let result = brawlers::table
            .filter(brawlers::username.eq(username))
            .select(BrawlerEntity::as_select())
            .first::<BrawlerEntity>(&mut connection)?;

        Ok(result)
    }

    async fn update_avatar(&self, _id: i32, avatar_url: String) -> Result<String> {
        // TODO: Implement actual database update when schema supports avatar
        Ok(avatar_url)
    }

    async fn crew_counting(&self, mission_id: i32) -> Result<u32> {
        let mut connection = Arc::clone(&self.db_pool).get()?;
        let count = crew_memberships::table
            .filter(crew_memberships::mission_id.eq(mission_id))
            .count()
            .get_result::<i64>(&mut connection)?;
        Ok(count as u32)
    }

    async fn get_missions(&self, brawler_id: i32) -> Result<Vec<MissionEntity>> {
        let mut connection = Arc::clone(&self.db_pool).get()?;
        
        let subquery = crew_memberships::table
            .filter(crew_memberships::brawler_id.eq(brawler_id))
            .select(crew_memberships::mission_id);

        let missions = missions::table
            .filter(
                missions::chief_id.eq(brawler_id)
                .or(missions::id.eq_any(subquery))
            )
            .filter(missions::deleted_at.is_null())
            .select(MissionEntity::as_select())
            .load(&mut connection)?;
        Ok(missions)
    }
}
