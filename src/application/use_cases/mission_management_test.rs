#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use crate::{
        application::use_cases::mission_management::MissionManagementUseCase,
        domain::{
            repositories::{
                mission_management::MockMissionManagementRepository,
                mission_viewing::MockMissionViewingRepository,
            },
            value_objects::mission_model::{NewMissionModel, UpdateMissionModel},
        },
    };

    #[tokio::test]
    async fn test_add() {
        let mut mock_mission_management_repository = MockMissionManagementRepository::new();
        let mock_mission_viewing_repository = MockMissionViewingRepository::new();

        mock_mission_management_repository
            .expect_add()
            .returning(|_| Box::pin(async { Ok(1) }));

        let mission_management_use_case = MissionManagementUseCase::new(
            Arc::new(mock_mission_management_repository),
            Arc::new(mock_mission_viewing_repository),
        );
        let add_mission_model = NewMissionModel {
            name: "Test Mission".to_string(),
            description: Some("Test Description".to_string()),
        };

        let result = mission_management_use_case
            .add(1, add_mission_model)
            .await
            .unwrap();
        assert_eq!(result, 1);
    }

    #[tokio::test]
    async fn test_edit_success() {
        let mut mock_mission_management_repository = MockMissionManagementRepository::new();
        let mut mock_mission_viewing_repository = MockMissionViewingRepository::new();

        mock_mission_viewing_repository
            .expect_crew_counting()
            .returning(|_| Box::pin(async { Ok(0) }));

        mock_mission_management_repository
            .expect_edit()
            .returning(|_, _| Box::pin(async { Ok(1) }));

        let mission_management_use_case = MissionManagementUseCase::new(
            Arc::new(mock_mission_management_repository),
            Arc::new(mock_mission_viewing_repository),
        );
        let edit_model = UpdateMissionModel {
            name: None,
            status: None,
            description: Some("Test".to_string()),
        };

        let result = mission_management_use_case
            .edit(1, 1, edit_model)
            .await
            .unwrap();
        assert_eq!(result, 1);
    }

    #[tokio::test]
    async fn test_edit_failed() {
        let mock_mission_management_repository = MockMissionManagementRepository::new();
        let mut mock_mission_viewing_repository = MockMissionViewingRepository::new();

        mock_mission_viewing_repository
            .expect_crew_counting()
            .returning(|_| Box::pin(async { Ok(13) }));

        let mission_management_use_case = MissionManagementUseCase::new(
            Arc::new(mock_mission_management_repository),
            Arc::new(mock_mission_viewing_repository),
        );
        let edit_model = UpdateMissionModel {
            name: None,
            status: None,
            description: Some("Test".to_string()),
        };

        let result = mission_management_use_case.edit(1, 1, edit_model).await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_remove_success() {
        let mut mock_mission_management_repository = MockMissionManagementRepository::new();
        let mut mock_mission_viewing_repository = MockMissionViewingRepository::new();

        mock_mission_viewing_repository
            .expect_crew_counting()
            .returning(|_| Box::pin(async { Ok(0) }));

        mock_mission_management_repository
            .expect_remove()
            .returning(|_, _| Box::pin(async { Ok(()) }));
        let mission_management_use_case = MissionManagementUseCase::new(
            Arc::new(mock_mission_management_repository),
            Arc::new(mock_mission_viewing_repository),
        );

        let result = mission_management_use_case.remove(1, 1).await.unwrap();

        assert_eq!(result, ());
    }

    #[tokio::test]
    async fn test_remove_failed() {
    let mock_mission_management_repository = MockMissionManagementRepository::new();
    let mut mock_mission_viewing_repository = MockMissionViewingRepository::new();

    mock_mission_viewing_repository
        .expect_crew_counting()
        .returning(|_| Box::pin(async { Ok(9) }));

    let mission_management_use_case = MissionManagementUseCase::new(
        Arc::new(mock_mission_management_repository),
        Arc::new(mock_mission_viewing_repository),
    );

    let result = mission_management_use_case.remove(1, 1).await;
    assert!(result.is_err());
}

}