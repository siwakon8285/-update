use crate::{
    domain::repositories::brawlers::BrawlerRepository,
    infrastructure::{
        argon2,
        jwt::{authentication_model::LoginModel, jwt_model::Passport},
    },
};
use anyhow::Result;
use std::sync::Arc;

pub struct AuthenticationUseCase<T>
where
    T: BrawlerRepository + Send + Sync,
{
    brawler_repository: Arc<T>,
}

impl<T> AuthenticationUseCase<T>
where
    T: BrawlerRepository + Send + Sync,
{
    pub fn new(brawler_repository: Arc<T>) -> Self {
        Self { brawler_repository }
    }

    pub async fn login(&self, login_model: LoginModel) -> Result<Passport> {
        let username = login_model.username.clone();

        let brawler_entity = self.brawler_repository.find_by_username(username).await?;
        let hash_password = brawler_entity.password;
        let login_password = login_model.password;

        if !argon2::verify(login_password, hash_password)? {
            return Err(anyhow::anyhow!("Invalid password!"));
        }

        let passport = Passport::new(
            brawler_entity.id,
            brawler_entity.display_name,
            brawler_entity.avatar_url,
        )?;

        Ok(passport)
    }
}
