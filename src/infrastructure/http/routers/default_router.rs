use axum::{extract::Path, http::StatusCode, response::IntoResponse};

pub async fn health_check() -> impl IntoResponse {
    (StatusCode::OK, " All Right, I'am Good").into_response()
}

pub async fn error(Path(status_code_u16): Path<u16>) -> impl IntoResponse {
    let status_code = StatusCode::from_u16(status_code_u16).unwrap();
    (status_code, status_code.to_string()).into_response()
}
