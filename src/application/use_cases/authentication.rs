use crate::{
    domain::repositories::brawlers::BrawlerRepository,
    infrastructure::{
        argon2,
        jwt::{
            self,
            authentication_model::LoginModel,
            jwt_model::{self, Passport},
        },
    },
};
use anyhow::Result;
use chrono::{Duration, Utc};
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
        let secret = crate::config::config_loader::get_user_secret()?;
        let username = login_model.username.clone();

        let brawler = self.brawler_repository.find_by_username(username).await?;

        let hash_password = brawler.password;
        let login_password = login_model.password;

        if !argon2::verify(login_password, hash_password)? {
            return Err(anyhow::anyhow!("Invalid password!"));
        }

        let access_claims = jwt_model::Claims {
            sub: brawler.id.to_string(),
            exp: (Utc::now() + Duration::days(1)).timestamp() as usize,
            iat: Utc::now().timestamp() as usize,
        };

        let _refresh_claims = jwt_model::Claims {
            sub: brawler.id.to_string(),
            exp: (Utc::now() + Duration::days(7)).timestamp() as usize,
            iat: Utc::now().timestamp() as usize,
        };

        let access_token = jwt::generate_token(&secret, &access_claims)?;

        Ok(Passport { access_token })
    }
}
