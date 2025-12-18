use crate::config::config_loader::get_user_secret;
use crate::infrastructure;
use axum::{
    body::Body,
    http::{Request, StatusCode, header},
    middleware::Next,
    response::Response,
};

pub async fn authorization(mut req: Request<Body>, next: Next) -> Result<Response, StatusCode> {
    let header = req
        .headers()
        .get(header::COOKIE)
        .and_then(|cookie_header| cookie_header.to_str().ok())
        .ok_or(StatusCode::UNAUTHORIZED)?;

    let token = get_cookie_value(header, "token").ok_or(StatusCode::UNAUTHORIZED)?;

    let secret = get_user_secret().map_err(|_| StatusCode::UNAUTHORIZED)?;

    let claims =
        infrastructure::jwt::verify_token(&secret, &token).map_err(|_| StatusCode::UNAUTHORIZED)?;

    let brawler_id = claims
        .sub
        .parse::<i32>()
        .map_err(|_| StatusCode::UNAUTHORIZED)?;


    req.extensions_mut().insert(brawler_id);

    Ok(next.run(req).await)
}

fn get_cookie_value(cookie_header: &str, key: &str) -> Option<String> {
    cookie_header.split("; ").find_map(|cookie| {
        let mut parts = cookie.splitn(2, '=');
        let name = parts.next()?.trim();
        let value = parts.next()?.trim();
        if name == key {
            Some(value.to_string())
        } else {
            None
        }
    })
}
