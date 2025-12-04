use anyhow::Result;
use async_trait::async_trait;
use diesel::{
    PgConnection,
    r2d2::{ConnectionManager, PooledConnection},
};
use mockall::automock;

use crate::domain::entities::crew_memberships::CrewMembershipEntity;

#[async_trait]
#[automock]
pub trait CrewOperationRepository {
    async fn join(&self, crew_memberships: CrewMembershipEntity) -> Result<()>;
    async fn leave(&self, crew_memberships: CrewMembershipEntity) -> Result<()>;

    //testing method
    fn for_insert_transaction_test(
        &self,
        conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
        crew_memberships: CrewMembershipEntity,
    ) -> Result<()>;
    fn for_delete_transaction_test(
        &self,
        conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
        crew_memberships: CrewMembershipEntity,
    ) -> Result<()>;

}