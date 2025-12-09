use anyhow::Result;
use std::sync::Arc;

use crate::domain::{
    repositories::{
        mission_operation::MissionOperationRepository, mission_viewing::MissionViewingRepository,
    },
    value_objects::mission_statuses::MissionStatuses,
};

pub struct MissionOperationUseCase<T1, T2>
where
    T1: MissionOperationRepository + Send + Sync,
    T2: MissionViewingRepository + Send + Sync,
{
    mission_operation_repository: Arc<T1>,
    mission_viewing_repository: Arc<T2>,
}

impl<T1, T2> MissionOperationUseCase<T1, T2>
where
    T1: MissionOperationRepository + Send + Sync,
    T2: MissionViewingRepository + Send + Sync,
{
    pub fn new(mission_operation_repository: Arc<T1>, mission_viewing_repository: Arc<T2>) -> Self {
        Self {
            mission_operation_repository,
            mission_viewing_repository,
        }
    }

    pub async fn in_progress(&self, mission_id: i32, chief_id: i32) -> Result<()> {
        let mission = self.mission_viewing_repository.get_one(mission_id).await?;

        let crew_count = self
            .mission_viewing_repository
            .crew_counting(mission_id)
            .await?;

        let is_status_open_or_fail = mission.status == MissionStatuses::Open.to_string()
            || mission.status == MissionStatuses::Failed.to_string();

        if !is_status_open_or_fail {
            return Err(anyhow::anyhow!("Mission status must be Open or Failed"));
        }

        if crew_count == 0 {
            return Err(anyhow::anyhow!(
                "Mission must have at least one crew member"
            ));
        }

        self.mission_operation_repository
            .in_progress(mission_id, chief_id)
            .await?;

        Ok(())
    }

    pub async fn to_completed(&self, mission_id: i32, chief_id: i32) -> Result<()> {
        self.mission_operation_repository
            .to_completed(mission_id, chief_id)
            .await?;
        Ok(())
    }

    pub async fn to_failed(&self, mission_id: i32, chief_id: i32) -> Result<()> {
        self.mission_operation_repository
            .to_failed(mission_id, chief_id)
            .await?;
        Ok(())
    }
}
