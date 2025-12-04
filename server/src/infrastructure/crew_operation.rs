use async_trait::async_trait;
use diesel::{
    ExpressionMethods, PgConnection, RunQueryDsl, delete, insert_into, r2d2::{ConnectionManager, PooledConnection}
};
use std::sync::Arc;
use crate::{domain::{entities::crew_memberships::CrewMembershipEntity, repositories::crew_operation::CrewOperationRepository}, infrastructure::database::{postgresql_connection::PgPoolSquad, schema::crew_memberships}};
use anyhow::Result;

pub struct CrewPostgres {
    db_pool: Arc<PgPoolSquad>,
}

impl CrewPostgres {
    pub fn new(db_pool: Arc<PgPoolSquad>) -> Self {
        Self { db_pool }
    }
}


#[async_trait]
impl CrewOperationRepository for CrewPostgres {


    async fn join(&self, crew_memberships: CrewMembershipEntity) -> Result<()> {
        let mut connection = Arc::clone(&self.db_pool).get()?;
        insert_into(crew_memberships::table)
            .values(crew_memberships)
            .execute(&mut connection)?;
        Ok(())
    }

    async fn leave(&self, crew_memberships: CrewMembershipEntity) -> Result<()> {
        let mut connection = Arc::clone(&self.db_pool).get()?;
        delete(crew_memberships::table)
            .filter(crew_memberships::brawler_id.eq(crew_memberships.brawler_id))
            .filter(crew_memberships::mission_id.eq(crew_memberships.mission_id))
            .execute(&mut connection)?;
        Ok(())
    }

    fn for_insert_transaction_test(
        &self,
        conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
        crew_memberships: CrewMembershipEntity,
    ) -> Result<()> {
        insert_into(crew_memberships::table)
            .values(crew_memberships)
            .execute(conn)?;

        Ok(())
    }

    fn for_delete_transaction_test(
        &self,
        conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
        crew_memberships: CrewMembershipEntity,
    ) -> Result<()> {
        delete(crew_memberships::table)
            .filter(crew_memberships::brawler_id.eq(crew_memberships.brawler_id))
            .filter(crew_memberships::mission_id.eq(crew_memberships.mission_id))
            .execute(conn)?;

        Ok(())
    }

}