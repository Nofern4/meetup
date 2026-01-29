use std::sync::Arc;

use crate::domain::{entities::crew_memberships::{CrewMembershipEntity, MAX_CREW_MEMBERSHIPS_PER_MISSION}, repositories::{
    crew_operation::CrewOperationRepository, mission_viewing::MissionViewingRepository,
    transaction_provider::TransactionProvider,
}, value_objects::mission_statuses::MissionStatuses};
use anyhow::Result;

pub struct CrewOperationUseCase<T1, T2, T3>
where
    T1: CrewOperationRepository + Send + Sync,
    T2: MissionViewingRepository + Send + Sync,
    T3: TransactionProvider + Send + Sync, // just a placeholder for the transaction provider, not necessarily.
{
    crew_operation_repository: Arc<T1>,
    mission_viewing_repository: Arc<T2>,
    tx: Arc<T3>,
}

impl<T1, T2, T3> CrewOperationUseCase<T1, T2, T3>
where
    T1: CrewOperationRepository + Send + Sync + 'static,
    T2: MissionViewingRepository + Send + Sync,
    T3: TransactionProvider + Send + Sync,
{
    pub fn new(
        crew_operation_repository: Arc<T1>,
        mission_viewing_repository: Arc<T2>,
        tx: Arc<T3>,
    ) -> Self {
        Self {
            crew_operation_repository,
            mission_viewing_repository,
            tx,
        }
    }

    pub async fn join(&self, mission_id: i32, brawler_id: i32) -> Result<()> {
        let mission = self
            .mission_viewing_repository
            .view_detail(mission_id)
            .await?;

        let crew_count = self
            .mission_viewing_repository
            .crew_counting(mission_id)
            .await?;

        let mission_status_condition = mission.status == MissionStatuses::Open.to_string()
            || mission.status == MissionStatuses::Failed.to_string();
        if !mission_status_condition {
            return Err(anyhow::anyhow!("Mission is not joinable"));
        }

        let crew_count_condition = crew_count < MAX_CREW_MEMBERSHIPS_PER_MISSION;
        if !crew_count_condition {
            return Err(anyhow::anyhow!("Mission is full"));
        }

        self.crew_operation_repository
            .join(CrewMembershipEntity {
                mission_id,
                brawler_id,
            })
            .await?;

        Ok(())
    }



    pub async fn leave(&self, mission_id: i32, brawler_id: i32) -> Result<()> {
        let mission = self
            .mission_viewing_repository
            .view_detail(mission_id)
            .await?;

        let leaving_condition = mission.status == MissionStatuses::Open.to_string()
            || mission.status == MissionStatuses::Failed.to_string();
        if !leaving_condition {
            return Err(anyhow::anyhow!("Mission is not leavable"));
        }

        self.crew_operation_repository
            .leave(CrewMembershipEntity {
                mission_id,
                brawler_id,
            })
            .await?;

        Ok(())
    }


    pub async fn join_and_delete_transaction(&self, mission_id: i32, brawler_id: i32) -> Result<()> {
        let tx = Arc::clone(&self.tx);
        let repo = Arc::clone(&self.crew_operation_repository);

        tx.transaction::<_, anyhow::Error, _>(move |conn| {
             repo.for_insert_transaction_test(
                conn,
                CrewMembershipEntity {
                    mission_id,
                    brawler_id,
                },
            )?;

            repo.for_delete_transaction_test(
                conn,
                CrewMembershipEntity {
                    mission_id,
                    brawler_id,
                },
            )?;
            
            Ok(())
        })
    }

}

