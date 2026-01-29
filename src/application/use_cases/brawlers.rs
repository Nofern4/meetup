use std::sync::Arc;
use crate::domain::value_objects::{brawler_model::RegisterBrawlerModel, mission_model::MissionModel};
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
        // Check if username already exists
        if let Ok(_) = self.brawler_repository.find_by_username(register_brawler_model.username.clone()).await {
            return Err(anyhow::anyhow!("Username already exists"));
        }

        let hashed_password = hash(register_brawler_model.password.clone())?;

        register_brawler_model.password = hashed_password;

        let register_entity = register_brawler_model.to_entity();

        let id = self.brawler_repository.register(register_entity).await?;

        Ok(id)
    }

    pub async fn upload_avatar(&self, base64_string: String, id: i32) -> Result<String, anyhow::Error> {
        // For now, just pass the base64 string or a dummy URL
        // In a real app, you would upload to S3 or save to disk and return the URL
        let avatar_url = format!("data:image/png;base64,{}", base64_string); 
        self.brawler_repository.update_avatar(id, avatar_url.clone()).await?;
        Ok(avatar_url)
    }

    pub async fn get_my_missions(&self, brawler_id: i32) -> Result<Vec<MissionModel>, anyhow::Error> {
        let missions = self.brawler_repository.get_missions(brawler_id).await?;
        let mut mission_models = Vec::new();
        for mission in missions {
            let count = self.brawler_repository.crew_counting(mission.id).await?;
            mission_models.push(mission.to_model(count as i64));
        }
        Ok(mission_models)
    }
}