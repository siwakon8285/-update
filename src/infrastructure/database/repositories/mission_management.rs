use anyhow::Result;
use async_trait::async_trait;
use diesel::prelude::*;
use std::sync::Arc;

use crate::{
    domain::{
        entities::missions::{AddMissionEntity, EditMissionEntity},
        repositories::mission_management::MissionManagementRepository,
        value_objects::mission_statuses::MissionStatuses,
    },
    infrastructure::database::{postgresql_connection::PgPoolSquad, schema::missions},
};

pub struct MissionManagementPostgres {
    db_pool: Arc<PgPoolSquad>,
}

impl MissionManagementPostgres {
    pub fn new(db_pool: Arc<PgPoolSquad>) -> Self {
        Self { db_pool }
    }
}

#[async_trait]
impl MissionManagementRepository for MissionManagementPostgres {
    async fn add(&self, add_mission_entity: AddMissionEntity) -> Result<i32> {
        let mut conn = self.db_pool.get()?;
        let result: i32 = diesel::insert_into(missions::table)
            .values(&add_mission_entity)
            .returning(missions::id)
            .get_result::<i32>(&mut conn)?;
        Ok(result)
    }

    async fn edit(&self, mission_id: i32, edit_mission_entity: EditMissionEntity) -> Result<i32> {
        let mut conn = self.db_pool.get()?;
        let result = diesel::update(missions::table)
            .filter(missions::id.eq(mission_id))
            .filter(missions::deleted_at.is_null())
            .filter(missions::status.eq(MissionStatuses::Open.to_string()))
            .set(&edit_mission_entity)
            .returning(missions::id)
            .get_result::<i32>(&mut conn)?;
        Ok(result)
    }

    async fn remove(&self, mission_id: i32, chief_id: i32) -> Result<()> {
        let mut conn = self.db_pool.get()?;

        diesel::update(missions::table)
            .filter(missions::id.eq(mission_id))
            .filter(missions::deleted_at.is_null())
            .filter(missions::status.eq(MissionStatuses::Open.to_string()))
            .set((
                missions::deleted_at.eq(diesel::dsl::now),
                missions::chief_id.eq(chief_id),
            ))
            .execute(&mut conn)?;

        Ok(())
    }
}
