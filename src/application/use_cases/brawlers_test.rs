#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use diesel;

    use crate::{
        application::use_cases::brawlers::BrawlersUseCase,
        domain::{
            repositories::brawlers::MockBrawlerRepository,
            value_objects::brawler_model::RegisterBrawlerModel,
        },
    };

    #[tokio::test]
    async fn test_brawler_register() {
        let mut mock_brawler_repository = MockBrawlerRepository::new();

        mock_brawler_repository
            .expect_register()
            .returning(|_| Box::pin(async { Ok(1) }));
        
        mock_brawler_repository
            .expect_find_by_username()
            .returning(|_| Box::pin(async { Err(anyhow::Error::from(diesel::result::Error::NotFound)) }));

        let brawlers_use_case = BrawlersUseCase::new(Arc::new(mock_brawler_repository));

        let register_brawler_model = RegisterBrawlerModel {
            username: "menta".to_string(),
            password: "P@ssw0rd".to_string(),
            display_name: "menta".to_string()
        };

        let result = brawlers_use_case
            .register(register_brawler_model)
            .await
            .unwrap();

        assert_eq!(result, 1);
    }
}
