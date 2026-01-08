use crate::infrastructure::http::middleware::auth::authorization;
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
use axum::{
    Extension, Json, Router,
    extract::{Path, State},
    response::IntoResponse,
    routing::post,
};
use std::sync::Arc;

pub fn routes(db_pool: Arc<PgPoolSquad>) -> Router {
    let repository = BrawlerPostgres::new(db_pool.clone());
    let use_case = BrawlersUseCase::new(Arc::new(repository));
    let state = Arc::new(use_case);

    let protected_routes = Router::new()
        .route("/avatar", post(upload_avatar))
        .route_layer(axum::middleware::from_fn(authorization));

    Router::new()
        .route("/register", post(register))
        .route(
            "/missions/{mission_id}/brawlers",
            axum::routing::get(get_brawlers_by_mission_id),
        )
        .merge(protected_routes)
        .with_state(state)
}

pub async fn register<T>(
    State(brawlers_use_case): State<Arc<BrawlersUseCase<T>>>,
    Json(register_brawler_model): Json<RegisterBrawlerModel>,
) -> impl IntoResponse
where
    T: BrawlerRepository + Send + Sync,
{
    match brawlers_use_case.register(register_brawler_model).await {
        Ok(passport) => (axum::http::StatusCode::CREATED, Json(passport)).into_response(),
        Err(e) => (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

pub async fn upload_avatar<T>(
    State(brawlers_use_case): State<Arc<BrawlersUseCase<T>>>,
    Extension(brawler_id): Extension<i32>,
    Json(upload_avatar): Json<crate::domain::value_objects::uploaded_image::UploadBase64Image>,
) -> impl IntoResponse
where
    T: BrawlerRepository + Send + Sync,
{
    match brawlers_use_case
        .upload_avatar(
            brawler_id,
            upload_avatar.base64_string,
            crate::domain::value_objects::uploaded_image::UploadImageOptions {
                folder: None,
                public_id: None,
                transformation: None,
            },
        )
        .await
    {
        Ok(uploaded_img) => (axum::http::StatusCode::OK, Json(uploaded_img)).into_response(),
        Err(e) => (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

pub async fn get_brawlers_by_mission_id<T>(
    State(brawlers_use_case): State<Arc<BrawlersUseCase<T>>>,
    Path(mission_id): Path<i32>,
) -> impl IntoResponse
where
    T: BrawlerRepository + Send + Sync,
{
    match brawlers_use_case
        .get_brawlers_by_mission_id(mission_id)
        .await
    {
        Ok(brawlers) => (axum::http::StatusCode::OK, Json(brawlers)).into_response(),
        Err(e) => (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}
