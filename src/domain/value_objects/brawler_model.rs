use crate::domain::entities::brawlers::RegisterBrawlerEntity;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegisterBrawlerModel {
    pub username: String,
    pub password: String,
}

impl RegisterBrawlerModel {
    pub fn to_entity(&self) -> RegisterBrawlerEntity {
        RegisterBrawlerEntity {
            username: self.username.clone(),
            password: self.password.clone(),
        }
    }
}
