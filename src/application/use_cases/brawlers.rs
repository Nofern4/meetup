use std::sync::Arc;
use crate::domain::value_objects::brawler_model::RegisterBrawlerModel;
use crate::infrastructure::argon2::hash;
use crate::domain::repositories::brawlers::BrawlerRepository;

pub struct BrawlersUseCase<T>
where
    T: BrawlerRepository + Send + Sync,
{
    brawler_repository: Arc<T>,
}


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