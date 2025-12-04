#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use crate::{
        application::use_cases::mission_viewing::MissionViewingUseCase,
        domain::{
            entities::missions::MissionEntity,
            repositories::mission_viewing::MockMissionViewingRepository,
            value_objects::{mission_filter::MissionFilter, mission_model::MissionModel, mission_statuses::MissionStatuses},
        },
    };

    #[tokio::test]
    async fn test_view_details() {
        let mut mock_mission_viewing_repository = MockMissionViewingRepository::new();

        mock_mission_viewing_repository
            .expect_crew_counting()
            .returning(|_| Box::pin(async { Ok(2) }));

        let now = chrono::Utc::now().naive_utc();

        mock_mission_viewing_repository
            .expect_view_detail()
            .returning(move |mission_id| {
                Box::pin(async move {
                    Ok(MissionEntity {
                        id: mission_id,
                        name: "Test".to_string(),
                        description: None,
                        create_at: now,
                        update_at: now,
                        chief_id: 1,
                        status: MissionStatuses::Open.to_string(),
                    })
                })
            });
        let want = MissionModel {
            id: 98,
            name: "Test".to_string(),
            description: None,
            created_at: now,
            updated_at: now,
            chief_id: 1,
            crew_count: 2,
            status: MissionStatuses::Open.to_string(),
        };

        let use_case = MissionViewingUseCase::new(Arc::new(mock_mission_viewing_repository));
        let result = use_case.view_detail(98).await.unwrap();
        assert_eq!(result, want)
    }

    #[tokio::test]
    async fn test_get() {
        let mut mock_mission_viewing_repository = MockMissionViewingRepository::new();

        mock_mission_viewing_repository
            .expect_crew_counting()
            .returning(|_| Box::pin(async { Ok(2) }));
        mock_mission_viewing_repository.expect_get().returning(|_| {
            Box::pin(async {
                let now = chrono::Utc::now().naive_utc();
                Ok(vec![MissionEntity {
                    id: 1,
                    name: "Test 1".to_string(),
                    description: None,
                    create_at: now,
                    update_at: now,
                    chief_id: 1,
                    status: MissionStatuses::Open.to_string(),
                }])
            })
        });
        let use_case = MissionViewingUseCase::new(Arc::new(mock_mission_viewing_repository));

        let result = use_case.get(&MissionFilter::default()).await.unwrap();

        assert_eq!(result.len(), 1)
    }
}