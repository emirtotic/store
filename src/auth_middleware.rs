use axum::{
    body::Body,
    extract::Request,
    http::StatusCode,
    middleware::Next,
    response::Response,
};
use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm};
use serde::Deserialize;
use std::{env, future::Future, pin::Pin};
use axum::http::HeaderMap;

#[derive(Debug, Deserialize, Clone)]
pub struct Claims {
    pub sub: String,
    pub role: String,
    pub exp: usize,
}

pub async fn require_auth(
    mut req: Request<Body>,
    next: Next,
) -> Result<Response, StatusCode> {
    let headers: &HeaderMap = req.headers();
    let auth_header = headers
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .ok_or(StatusCode::UNAUTHORIZED)?;

    let token = auth_header.strip_prefix("Bearer ").ok_or(StatusCode::UNAUTHORIZED)?;

    let secret = env::var("JWT_SECRET").unwrap_or_else(|_| "secret".into());

    let decoded = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::new(Algorithm::HS256),
    )
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

    req.extensions_mut().insert(decoded.claims);

    Ok(next.run(req).await)
}

pub fn require_role(
    role_required: &'static str,
) -> impl Fn(Request<Body>, Next) -> Pin<Box<dyn Future<Output = Result<Response, StatusCode>> + Send>> + Clone + Send + 'static {
    move |req: Request<Body>, next: Next| {
        let role_required = role_required.to_string();
        Box::pin(async move {
            let claims = req.extensions().get::<Claims>();

            match claims {
                Some(c) if c.role == role_required => Ok(next.run(req).await),
                Some(_) => Err(StatusCode::FORBIDDEN),
                None => Err(StatusCode::UNAUTHORIZED),
            }
        })
    }
}