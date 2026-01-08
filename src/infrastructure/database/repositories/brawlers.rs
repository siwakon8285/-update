use anyhow::Result;
use async_trait::async_trait;
use diesel::insert_into;
use diesel::prelude::*;
use std::sync::Arc;

use crate::{
    domain::{
        entities::brawlers::{BrawlerEntity, RegisterBrawlerEntity},
        repositories::brawlers::BrawlerRepository,
        value_objects::{
            base64_image::Base64Image,
            uploaded_image::{UploadImageOptions, UploadedImage},
        },
    },
    infrastructure::{
        cloudinary,
        database::{postgresql_connection::PgPoolSquad, schema::brawlers},
    },
};

pub struct BrawlerPostgres {
    db_pool: Arc<PgPoolSquad>,
}

impl BrawlerPostgres {
    pub fn new(db_pool: Arc<PgPoolSquad>) -> Self {
        Self { db_pool }
    }
}

#[async_trait]
impl BrawlerRepository for BrawlerPostgres {
    async fn register(&self, register_brawler_entity: RegisterBrawlerEntity) -> Result<i32> {
        let mut connection = Arc::clone(&self.db_pool).get()?;

        let result = insert_into(brawlers::table)
            .values(&register_brawler_entity)
            .returning(brawlers::id)
            .get_result::<i32>(&mut connection)?;

        Ok(result)
    }

    async fn find_by_username(&self, username: String) -> Result<BrawlerEntity> {
        let mut connection = Arc::clone(&self.db_pool).get()?;

        let result = brawlers::table
            .filter(brawlers::username.eq(username))
            .select(BrawlerEntity::as_select())
            .first::<BrawlerEntity>(&mut connection)?;

        Ok(result)
    }

    async fn upload_avatar(
        &self,
        brawler_id: i32,
        base64_image: Base64Image,
        option: UploadImageOptions,
    ) -> Result<UploadedImage> {
        let uploaded_image = cloudinary::upload(base64_image, option).await?;

        let mut connection = Arc::clone(&self.db_pool).get()?;

        diesel::update(brawlers::table.filter(brawlers::id.eq(brawler_id)))
            .set((
                brawlers::avatar_url.eq(Some(uploaded_image.url.clone())),
                brawlers::avatar_public_id.eq(Some(uploaded_image.public_id.clone())),
            ))
            .execute(&mut connection)?;

        Ok(uploaded_image)
    }

    async fn get_brawlers_by_mission_id(&self, mission_id: i32) -> Result<Vec<BrawlerEntity>> {
        use crate::infrastructure::database::schema::crew_memberships;

        let mut connection = Arc::clone(&self.db_pool).get()?;

        let result = brawlers::table
            .inner_join(crew_memberships::table)
            .filter(crew_memberships::mission_id.eq(mission_id))
            .select(BrawlerEntity::as_select())
            .load::<BrawlerEntity>(&mut connection)?;

        Ok(result)
    }
}
