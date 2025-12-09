use crate::domain::entities::crew_memberships::CrewMemberShips;
use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait CrewOperationRepository {
    async fn join(&self, crew_memberships: CrewMemberShips) -> Result<()>;
    async fn leave(&self, crew_memberships: CrewMemberShips) -> Result<()>;
}
