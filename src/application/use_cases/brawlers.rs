use crate::{
    domain::{
        entities::brawlers::BrawlerEntity,
        repositories::brawlers::BrawlerRepository,
        value_objects::{
            base64_image::Base64Image,
            brawler_model::RegisterBrawlerModel,
            uploaded_image::{UploadImageOptions, UploadedImage},
        },
    },
    infrastructure::{argon2::hash, jwt::jwt_model::Passport},
};
use anyhow::Result;
use std::sync::Arc;

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

    pub async fn register(&self, mut register_model: RegisterBrawlerModel) -> Result<Passport> {
        register_model.password = hash(register_model.password.clone())?;

        let register_entity = register_model.to_entity();

        let brawler_id = self.brawler_repository.register(register_entity).await?;

        let passport = Passport::new(brawler_id, register_model.display_name.clone(), None)?;

        Ok(passport)
    }

    pub async fn upload_avatar(
        &self,
        brawler_id: i32,
        base64_image: String,
        _option: UploadImageOptions,
    ) -> Result<UploadedImage> {
        let option = UploadImageOptions {
            folder: Some("brawlers_avatar".to_string()),
            public_id: Some(brawler_id.to_string()),
            transformation: Some("c_scale, w_256".to_string()),
        };
        let base64_image = Base64Image::new(base64_image)?;

        let uploaded = self
            .brawler_repository
            .upload_avatar(brawler_id, base64_image, option)
            .await?;
        Ok(uploaded)
    }

    pub async fn get_brawlers_by_mission_id(&self, mission_id: i32) -> Result<Vec<BrawlerEntity>> {
        self.brawler_repository
            .get_brawlers_by_mission_id(mission_id)
            .await
    }
}
