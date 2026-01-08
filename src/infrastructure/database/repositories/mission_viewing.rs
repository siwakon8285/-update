use anyhow::Result;
use async_trait::async_trait;
use diesel::prelude::*;
use std::sync::Arc;

use crate::domain::{
    entities::missions::MissionEntity,
    repositories::mission_viewing::MissionViewingRepository,
    value_objects::{brawler_model::BrawlerModel, mission_filter::MissionFilter},
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

    async fn get_mission_crew(&self, mission_id: i32) -> Result<Vec<BrawlerModel>> {
        let mut conn = self.db_pool.get()?;

        let sql = r#"
            SELECT 
                b.display_name,
                COALESCE(b.avatar_url, '') AS avatar_url,
                COALESCE(s.success_count, 0) AS mission_success_count,
                COALESCE(j.joined_count, 0) AS mission_joined_count
            FROM crew_memberships cm
            INNER JOIN brawlers b ON b.id = cm.brawler_id
            LEFT JOIN (
                SELECT cm2.brawler_id, COUNT(*) AS success_count
                FROM crew_memberships cm2
                INNER JOIN missions m2 ON m2.id = cm2.mission_id
                WHERE m2.status = 'SUCCESS'
                GROUP BY cm2.brawler_id
            ) s ON s.brawler_id = b.id
            LEFT JOIN (
                SELECT cm3.brawler_id, COUNT(*) AS joined_count
                FROM crew_memberships cm3
                GROUP BY cm3.brawler_id
            ) j ON j.brawler_id = b.id
            WHERE cm.mission_id = $1
        "#;

        let results = diesel::sql_query(sql)
            .bind::<diesel::sql_types::Int4, _>(mission_id)
            .load::<BrawlerModel>(&mut conn)?;

        Ok(results)
    }
}
