use anyhow::Result;
use async_trait::async_trait;
use diesel::prelude::*;
use std::sync::Arc;

use crate::domain::{
    entities::{crew_memberships::CrewMemberShips, missions::MissionEntity},
    repositories::mission_viewing::MissionViewingRepository,
    value_objects::{mission_filter::MissionFilter, mission_model::MissionModel},
};
use crate::infrastructure::database::{
    postgresql_connection::PgPoolSquad,
    schema::{crew_memberships, missions},
};

pub struct MissionViewingPostgres {
    db_pool: Arc<PgPoolSquad>,
}

impl MissionViewingPostgres {
    pub fn new(db_pool: Arc<PgPoolSquad>) -> Self {
        Self { db_pool }
    }
}

#[async_trait]
impl MissionViewingRepository for MissionViewingPostgres {
    async fn crew_counting(&self, mission_id: i32) -> Result<u32> {
        let mut conn = self.db_pool.get()?;

        let count = crew_memberships::table
            .filter(crew_memberships::mission_id.eq(mission_id))
            .count()
            .get_result::<i64>(&mut conn)?;

        Ok(count as u32)
    }

    async fn get_one(&self, mission_id: i32) -> Result<MissionEntity> {
        let mut conn = self.db_pool.get()?;
        let result = missions::table
            .filter(missions::id.eq(mission_id))
            .filter(missions::deleted_at.is_null())
            .select(MissionEntity::as_select())
            .first::<MissionEntity>(&mut conn)?;
        Ok(result)
    }

    async fn get_all(&self, filter: &MissionFilter) -> Result<Vec<MissionEntity>> {
        let mut conn = self.db_pool.get()?;

        let mut query = missions::table
            .filter(missions::deleted_at.is_null())
            .into_boxed();

        if let Some(status) = &filter.status {
            query = query.filter(missions::status.eq(status.to_string()));
        };

        if let Some(name) = &filter.name {
            query = query.filter(missions::name.ilike(format!("%{}%", name)));
        };

        let results = query
            .select(MissionEntity::as_select())
            .order_by(missions::created_at.desc())
            .load::<MissionEntity>(&mut conn)?;

        Ok(results)
    }
}
