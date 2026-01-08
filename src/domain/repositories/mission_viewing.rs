use crate::domain::{
    entities::missions::MissionEntity, value_objects::brawler_model::BrawlerModel,
    value_objects::mission_filter::MissionFilter,
};
use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait MissionViewingRepository {
    async fn crew_counting(&self, mission_id: i32) -> Result<u32>;
    async fn get_one(&self, mission_id: i32) -> Result<MissionEntity>;
    async fn get_all(&self, filter: &MissionFilter) -> Result<Vec<MissionEntity>>;
    async fn get_mission_crew(&self, mission_id: i32) -> Result<Vec<BrawlerModel>>;
}
