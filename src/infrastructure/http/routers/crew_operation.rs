use axum::{
    Extension, Router,
    extract::{Path, State},
    http::StatusCode,
    middleware,
    response::IntoResponse,
    routing::{delete, post},
};
use std::sync::Arc;

use crate::{
    application::use_cases::crew_operation::CrewOperationUseCase,
    domain::repositories::{
        crew_operation::CrewOperationRepository, mission_viewing::MissionViewingRepository,
    },
    infrastructure::{
        database::{
            postgresql_connection::PgPoolSquad,
            repositories::{
                crew_operation::CrewOperationPostgres, mission_viewing::MissionViewingPostgres,
            },
        },
        http::middleware::auth::authorization,
    },
};

pub async fn join<T1, T2>(
    State(use_case): State<Arc<CrewOperationUseCase<T1, T2>>>,
    Extension(user_id): Extension<i32>,
    Path(mission_id): Path<i32>,
) -> impl IntoResponse
where
    T1: CrewOperationRepository + Send + Sync + 'static,
    T2: MissionViewingRepository + Send + Sync,
{
    match use_case.join(mission_id, user_id).await {
        Ok(_) => (
            StatusCode::OK,
            format!("Join Mission_id:{} completed", mission_id),
        )
            .into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

pub async fn leave<T1, T2>(
    State(use_case): State<Arc<CrewOperationUseCase<T1, T2>>>,
    Extension(user_id): Extension<i32>,
    Path(mission_id): Path<i32>,
) -> impl IntoResponse
where
    T1: CrewOperationRepository + Send + Sync + 'static,
    T2: MissionViewingRepository + Send + Sync,
{
    match use_case.leave(mission_id, user_id).await {
        Ok(_) => (
            StatusCode::OK,
            format!("Leave Mission_id:{} completed", mission_id),
        )
            .into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

pub fn routes(db_pool: Arc<PgPoolSquad>) -> Router {
    let crew_repo = CrewOperationPostgres::new(db_pool.clone());
    let viewing_repo = MissionViewingPostgres::new(db_pool);
    let use_case = CrewOperationUseCase::new(Arc::new(crew_repo), Arc::new(viewing_repo));

    Router::new()
        .route("/join/{mission_id}", post(join))
        .route("/leave/{mission_id}", delete(leave))
        .route_layer(middleware::from_fn(authorization))
        .with_state(Arc::new(use_case))
}
