use crate::models::{LoginRequest, RegisterUser, Role, User, UserResponse};
use axum::{extract::State, http::StatusCode, Json};
use bcrypt::{hash, verify};
use chrono::Utc;
use jsonwebtoken::{encode, EncodingKey, Header};
use sqlx::MySqlPool;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub role: String,
    pub exp: usize,
}

pub async fn register_user(
    State(pool): State<MySqlPool>,
    Json(data): Json<RegisterUser>,
) -> Result<Json<UserResponse>, StatusCode> {
    tracing::info!("Registering new user: {}", data.username);

    let role = sqlx::query_as!(
        Role,
        r#"SELECT id, name FROM roles WHERE name = ?"#,
        data.role
    )
    .fetch_optional(&pool)
    .await
    .map_err(|e| {
        tracing::error!("DB error (role check): {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let role = match role {
        Some(r) => r,
        None => {
            tracing::warn!("Invalid role: {}", data.role);
            return Err(StatusCode::BAD_REQUEST);
        }
    };

    let password_hash = hash(&data.password, 10).map_err(|e| {
        tracing::error!("Password hashing failed: {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let result = sqlx::query!(
        r#"
        INSERT INTO users (username, password_hash, role_id)
        VALUES (?, ?, ?)
        "#,
        data.username,
        password_hash,
        role.id
    )
    .execute(&pool)
    .await
    .map_err(|e| {
        tracing::error!("Insert failed: {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let user_id = result.last_insert_id() as i64;

    let user = sqlx::query_as!(
        User,
        r#"
        SELECT id, username, password_hash, role_id
        FROM users
        WHERE id = ?
        "#,
        user_id
    )
    .fetch_one(&pool)
    .await
    .map_err(|e| {
        tracing::error!("Fetch inserted user failed: {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let response = UserResponse {
        id: user.id,
        username: user.username,
        role_id: user.role_id,
    };

    Ok(Json(response))
}

pub async fn login_user(
    State(pool): State<MySqlPool>,
    Json(data): Json<LoginRequest>,
) -> Result<Json<String>, StatusCode> {
    tracing::info!("Login attempt for {}", data.username);

    let user = sqlx::query_as!(
        User,
        r#"
        SELECT id, username, password_hash, role_id
        FROM users
        WHERE username = ?
        "#,
        data.username
    )
    .fetch_optional(&pool)
    .await
    .map_err(|e| {
        tracing::error!("DB error: {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let user = match user {
        Some(u) => u,
        None => {
            tracing::warn!("User not found: {}", data.username);
            return Err(StatusCode::UNAUTHORIZED);
        }
    };

    let is_valid = verify(&data.password, &user.password_hash)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if !is_valid {
        tracing::warn!("Invalid password for user: {}", data.username);
        return Err(StatusCode::UNAUTHORIZED);
    }

    let expiration = Utc::now()
        .checked_add_signed(chrono::Duration::minutes(20))
        .expect("valid timestamp")
        .timestamp() as usize;

    let role = sqlx::query_scalar!(r#"SELECT name FROM roles WHERE id = ?"#, user.role_id)
        .fetch_one(&pool)
        .await
        .map_err(|e| {
            tracing::error!("Failed to get role name: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    let claims = Claims {
        sub: user.username.clone(),
        role, // sada je ovo npr. "seller" ili "customer"
        exp: expiration,
    };

    let secret = std::env::var("JWT_SECRET").unwrap_or_else(|_| "secret".into());

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )
    .map_err(|e| {
        tracing::error!("Token generation failed: {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Json(token))
}
