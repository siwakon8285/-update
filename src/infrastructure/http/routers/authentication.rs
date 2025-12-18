use std::sync::Arc;

use axum::{Json, Router, extract::State, http::StatusCode, response::IntoResponse, routing::post};

use crate::{
    application::use_cases::authentication::AuthenticationUseCase,
    domain::repositories::brawlers::BrawlerRepository,
    infrastructure::{
        database::{postgresql_connection::PgPoolSquad, repositories::brawlers::BrawlerPostgres},
        jwt::authentication_model::LoginModel,
    },
};

pub async fn login<T>(
    State(user_case): State<Arc<AuthenticationUseCase<T>>>,
    Json(model): Json<LoginModel>,
) -> impl IntoResponse
where
    T: BrawlerRepository + Send + Sync,
{
    match user_case.login(model).await {
        Ok(passport) => (StatusCode::OK, Json(passport)).into_response(),
        Err(e) => {
            let error_message = e.to_string();
            let error_lower = error_message.to_lowercase();
            if error_lower.contains("invalid password") || error_lower.contains("not found") {
                (StatusCode::UNAUTHORIZED, error_message).into_response()
            } else {
                (StatusCode::INTERNAL_SERVER_ERROR, error_message).into_response()
            }
        }
    }
}

pub fn routes(db_pool: Arc<PgPoolSquad>) -> Router {
    let repository = BrawlerPostgres::new(db_pool.clone());
    let use_case = AuthenticationUseCase::new(Arc::new(repository));

    Router::new()
        .route("/login", post(login))
        .with_state(Arc::new(use_case))
}
