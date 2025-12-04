#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use chrono::{TimeZone, Utc};
    use diesel::{PgConnection, r2d2::{ConnectionManager, PooledConnection}};

    use crate::{application::use_cases::crew_operation::CrewOperationUseCase, domain::{entities::{crew_memberships::MAX_CREW_MEMBERSHIPS_PER_MISSION, missions::MissionEntity}, repositories::{crew_operation::MockCrewOperationRepository, mission_viewing::MockMissionViewingRepository, transaction_provider::MockTransactionProvider}, value_objects::mission_statuses::MissionStatuses}, infrastructure::database::postgresql_connection::establish_connection};

    #[tokio::test]
    async fn test_join_success() {
        let mut mock_crew_repo = MockCrewOperationRepository::new();
        let mut mock_mission_repo = MockMissionViewingRepository::new();
        let mock_tx_repo = MockTransactionProvider::new();

        let now = Utc
            .with_ymd_and_hms(1970, 1, 1, 0, 0, 0)
            .unwrap()
            .naive_utc();

        mock_mission_repo
            .expect_crew_counting()
            .returning(|_| Box::pin(async { Ok(2) }));
        mock_mission_repo.expect_view_detail().returning(move |_| {
            Box::pin(async move {
                Ok(MissionEntity {
                    id: 1,
                    name: "test mission".to_string(),
                    description: Some("test".to_string()),
                    status: MissionStatuses::Open.to_string(),
                    chief_id: 1,
                    create_at: now,
                    update_at: now,
                })
            })
        });
        mock_crew_repo
            .expect_join()
            .returning(|_| Box::pin(async { Ok(()) }));

        let use_case = CrewOperationUseCase::new(
            Arc::new(mock_crew_repo),
            Arc::new(mock_mission_repo),
            Arc::new(mock_tx_repo),
        );

        let result = use_case.join(1, 1).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_join_fails_when_mission_is_not_open() {
        #[allow(unused_mut)]
        let mut mock_crew_repo = MockCrewOperationRepository::new();
        let mut mock_mission_repo = MockMissionViewingRepository::new();
        let mock_tx_repo = MockTransactionProvider::new();

        let now = Utc
            .with_ymd_and_hms(1970, 1, 1, 0, 0, 0)
            .unwrap()
            .naive_utc();

        mock_mission_repo
            .expect_crew_counting()
            .returning(|_| Box::pin(async { Ok(2) }));
        mock_mission_repo.expect_view_detail().returning(move |_| {
            Box::pin(async move {
                Ok(MissionEntity {
                    id: 1,
                    name: "test mission".to_string(),
                    description: Some("test".to_string()),
                    status: MissionStatuses::InProgress.to_string(),
                    chief_id: 1,
                    create_at: now,
                    update_at: now,
                })
            })
        });
        let use_case = CrewOperationUseCase::new(
            Arc::new(mock_crew_repo),
            Arc::new(mock_mission_repo),
            Arc::new(mock_tx_repo),
        );

        let result = use_case.join(1, 1).await;

        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "Mission is not joinable");

    }


    #[tokio::test]
    async fn test_join_fails_when_mission_is_full() {
        #[allow(unused_mut)]
        let mut mock_crew_repo = MockCrewOperationRepository::new();
        let mut mock_mission_repo = MockMissionViewingRepository::new();
        let mock_tx_repo = MockTransactionProvider::new();

        let now = Utc
            .with_ymd_and_hms(1970, 1, 1, 0, 0, 0)
            .unwrap()
            .naive_utc();

        mock_mission_repo
            .expect_crew_counting()
            .returning(|_| Box::pin(async { Ok(MAX_CREW_MEMBERSHIPS_PER_MISSION) }));

        mock_mission_repo.expect_view_detail().returning(move |_| {
            Box::pin(async move {
                Ok(MissionEntity {
                    id: 1,
                    name: "test mission".to_string(),
                    description: Some("test".to_string()),
                    status: MissionStatuses::Open.to_string(),
                    chief_id: 1,
                    create_at: now,
                    update_at: now,
                })
            })
        });

        let use_case = CrewOperationUseCase::new(
            Arc::new(mock_crew_repo),
            Arc::new(mock_mission_repo),
            Arc::new(mock_tx_repo),
        );

        let result = use_case.join(1, 1).await;

        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "Mission is full");

    }

    #[tokio::test]
    async fn test_leave_success() {
        let mut mock_crew_repo = MockCrewOperationRepository::new();
        let mut mock_mission_repo = MockMissionViewingRepository::new();
        let mock_tx_repo = MockTransactionProvider::new();

        let now = Utc
            .with_ymd_and_hms(1970, 1, 1, 0, 0, 0)
            .unwrap()
            .naive_utc();
        mock_mission_repo.expect_view_detail().returning(move |_| {
            Box::pin(async move {
                Ok(MissionEntity {
                    id: 1,
                    name: "test mission".to_string(),
                    description: Some("test".to_string()),
                    status: MissionStatuses::Open.to_string(),
                    chief_id: 1,
                    create_at: now,
                    update_at: now,


                })
            })
        });
        mock_crew_repo
            .expect_leave()
            .returning(|_| Box::pin(async { Ok(()) }));

        let use_case = CrewOperationUseCase::new(
            Arc::new(mock_crew_repo),
            Arc::new(mock_mission_repo),
            Arc::new(mock_tx_repo),
        );

        let result = use_case.leave(1, 1).await;

        assert!(result.is_ok());
    }


    #[tokio::test]
    async fn test_leave_fails_when_mission_is_not_open() {
        let mut mock_crew_repo = MockCrewOperationRepository::new();
        let mut mock_mission_repo = MockMissionViewingRepository::new();
        let mock_tx_repo = MockTransactionProvider::new();

        let now = Utc
            .with_ymd_and_hms(1970, 1, 1, 0, 0, 0)
            .unwrap()
            .naive_utc();
        mock_mission_repo.expect_view_detail().returning(move |_| {
            Box::pin(async move {
                Ok(MissionEntity {
                    id: 1,
                    name: "test mission".to_string(),
                    description: Some("test".to_string()),
                    status: MissionStatuses::InProgress.to_string(),
                    chief_id: 1,
                    create_at: now,
                    update_at: now,
                })
            })
        });
        mock_crew_repo
            .expect_leave()
            .returning(|_| Box::pin(async { Ok(()) }));

        let use_case = CrewOperationUseCase::new(
            Arc::new(mock_crew_repo),
            Arc::new(mock_mission_repo),
            Arc::new(mock_tx_repo),
        );

        let result = use_case.leave(1, 1).await;

        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "Mission is not leavable")
    }


    // This is need a real database connection to test the transaction.
    #[tokio::test]
    async fn test_insert_and_delete_transaction() {
        let mut mock_crew_repo = MockCrewOperationRepository::new();
        let mock_mission_repo = MockMissionViewingRepository::new();
        let mut mock_tx_repo = MockTransactionProvider::new();

        mock_tx_repo.expect_transaction().returning(
            |f: Box<
                dyn FnOnce(
                        &mut PooledConnection<ConnectionManager<PgConnection>>,
                    ) -> Result<(), anyhow::Error>
                    + 'static,
            >| {
                dotenvy::dotenv().ok();
                let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL is invalid");
                let db_pool = establish_connection(&db_url).unwrap();
                let mut conn = db_pool.get().unwrap();
                f(&mut conn)
            },
        );
        mock_crew_repo
            .expect_for_insert_transaction_test()
            .returning(|_, _| Ok(()));

        mock_crew_repo
            .expect_for_delete_transaction_test()
            .returning(|_, _| Ok(()));

        let use_case = CrewOperationUseCase::new(
            Arc::new(mock_crew_repo),
            Arc::new(mock_mission_repo),
            Arc::new(mock_tx_repo),
        );

        let result = use_case.join_and_delete_transaction(1, 1).await;

        assert!(result.is_ok());
    }

            
}