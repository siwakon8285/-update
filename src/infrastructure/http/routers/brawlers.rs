use crate::{
    application::use_cases::brawlers::BrawlersUseCase,
    domain::{
        repositories::brawlers::BrawlerRepository,
        value_objects::brawler_model::RegisterBrawlerModel,
    },
    infrastructure::database::{
        postgresql_connection::PgPoolSquad, repositories::brawlers::BrawlerPostgres,
    },
};
use axum::{Json, Router, extract::State, http::StatusCode, response::IntoResponse, routing::post};
use std::sync::Arc;

pub fn routes(db_pool: Arc<PgPoolSquad>) -> Router {
    let repository = BrawlerPostgres::new(db_pool.clone());
    let use_case = BrawlersUseCase::new(Arc::new(repository));

    Router::new()
        .route("/register", post(register))
        .with_state(Arc::new(use_case))
}

pub async fn register<T>(
    State(brawlers_use_case): State<Arc<BrawlersUseCase<T>>>,
    Json(register_brawler_model): Json<RegisterBrawlerModel>,
) -> impl IntoResponse
where
    T: BrawlerRepository + Send + Sync,
{
    match brawlers_use_case.register(register_brawler_model).await {
        Ok(passport) => (StatusCode::CREATED, Json(passport)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}
