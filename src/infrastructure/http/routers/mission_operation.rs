use axum::{
    Extension, Router,
    extract::{Path, State},
    http::StatusCode,
    middleware,
    response::IntoResponse,
    routing::patch,
};
use std::sync::Arc;

use crate::{
    application::use_cases::mission_operation::MissionOperationUseCase,
    domain::repositories::{
        mission_operation::MissionOperationRepository, mission_viewing::MissionViewingRepository,
    },
    infrastructure::{
        database::{
            postgresql_connection::PgPoolSquad,
            repositories::{
                mission_operation::MissionOperationPostgres,
                mission_viewing::MissionViewingPostgres,
            },
        },
        http::middleware::auth::authorization,
    },
};

pub async fn in_progress<T1, T2>(
    State(use_case): State<Arc<MissionOperationUseCase<T1, T2>>>,
    Extension(user_id): Extension<i32>,
    Path(mission_id): Path<i32>,
) -> impl IntoResponse
where
    T1: MissionOperationRepository + Send + Sync + 'static,
    T2: MissionViewingRepository + Send + Sync,
{
    match use_case.in_progress(mission_id, user_id).await {
        Ok(id) => (StatusCode::OK, format!("Mission {} is now in progress", id)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

pub async fn to_completed<T1, T2>(
    State(use_case): State<Arc<MissionOperationUseCase<T1, T2>>>,
    Extension(user_id): Extension<i32>,
    Path(mission_id): Path<i32>,
) -> impl IntoResponse
where
    T1: MissionOperationRepository + Send + Sync + 'static,
    T2: MissionViewingRepository + Send + Sync,
{
    match use_case.to_completed(mission_id, user_id).await {
        Ok(id) => (StatusCode::OK, format!("Mission {} completed", id)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

pub async fn to_failed<T1, T2>(
    State(use_case): State<Arc<MissionOperationUseCase<T1, T2>>>,
    Extension(user_id): Extension<i32>,
    Path(mission_id): Path<i32>,
) -> impl IntoResponse
where
    T1: MissionOperationRepository + Send + Sync + 'static,
    T2: MissionViewingRepository + Send + Sync,
{
    match use_case.to_failed(mission_id, user_id).await {
        Ok(id) => (StatusCode::OK, format!("Mission {} failed", id)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

pub fn routes(db_pool: Arc<PgPoolSquad>) -> Router {
    let operation_repository = MissionOperationPostgres::new(db_pool.clone());
    let viewing_repository = MissionViewingPostgres::new(db_pool);
    let use_case =
        MissionOperationUseCase::new(Arc::new(operation_repository), Arc::new(viewing_repository));

    Router::new()
        .route("/in-progress/{mission_id}", patch(in_progress))
        .route("/to-completed/{mission_id}", patch(to_completed))
        .route("/to-failed/{mission_id}", patch(to_failed))
        .route_layer(middleware::from_fn(authorization))
        .with_state(Arc::new(use_case))
}
