use std::sync::Arc;
use crate::domain::value_objects::brawler_model::RegisterBrawlerModel;
use crate::infrastructure::argon2::hash;
use crate::domain::repositories::brawlers::BrawlerRepository;

#[allow(dead_code)]
pub struct BrawlersUseCase<T>
where
    T: BrawlerRepository + Send + Sync,
{
    brawler_repository: Arc<T>,
}

#[allow(dead_code)]
impl<T> BrawlersUseCase<T>
where
    T: BrawlerRepository + Send + Sync,
{
    pub fn new(brawler_repository: Arc<T>) -> Self {
        Self { brawler_repository }
    }

    pub async fn register(&self, mut register_brawler_model: RegisterBrawlerModel) -> Result<i32, anyhow::Error> {
        let hashed_password = hash(register_brawler_model.password.clone())?;

        register_brawler_model.password = hashed_password;

        let register_entity = register_brawler_model.to_entity();

        let id = self.brawler_repository.register(register_entity).await?;

        Ok(id)
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

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

        let brawlers_use_case = BrawlersUseCase::new(Arc::new(mock_brawler_repository));

        let register_brawler_model = RegisterBrawlerModel {
            username: "menta".to_string(),
            password: "P@ssw0rd".to_string(),
        };

        let result = brawlers_use_case
            .register(register_brawler_model)
            .await
            .unwrap();

        assert_eq!(result, 1);
    }
}