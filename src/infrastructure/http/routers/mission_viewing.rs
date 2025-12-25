use axum::{
    Json, Router,
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    routing::get,
};
use std::sync::Arc;

use crate::{
    application::use_cases::mission_viewing::MissionViewingUseCase,
    domain::{
        repositories::mission_viewing::MissionViewingRepository,
        value_objects::mission_filter::MissionFilter,
    },
    infrastructure::database::{
        postgresql_connection::PgPoolSquad, repositories::mission_viewing::MissionViewingPostgres,
    },
};

pub async fn get_one<T>(
    State(use_case): State<Arc<MissionViewingUseCase<T>>>,
    Path(mission_id): Path<i32>,
) -> impl IntoResponse
where
    T: MissionViewingRepository + Send + Sync + 'static,
{
    match use_case.view_detail(mission_id).await {
        Ok(model) => (StatusCode::OK, Json(model)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

pub async fn get_all<T>(
    State(use_case): State<Arc<MissionViewingUseCase<T>>>,
    Query(filter): Query<MissionFilter>,
) -> impl IntoResponse
where
    T: MissionViewingRepository + Send + Sync + 'static,
{
    match use_case.get(&filter).await {
        Ok(models) => (StatusCode::OK, Json(models)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

pub fn routes(db_pool: Arc<PgPoolSquad>) -> Router {
    let viewing_repository = MissionViewingPostgres::new(db_pool);
    let use_case = MissionViewingUseCase::new(Arc::new(viewing_repository));

    Router::new()
        .route("/", get(get_all))
        .route("/{mission_id}", get(get_one))
        .route("/filter", get(get_all))
        .with_state(Arc::new(use_case))
}
