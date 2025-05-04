use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Category {
    pub id: i64,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateCategory {
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateCategory {
    pub name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Item {
    pub id: i64,
    pub name: String,
    pub price: f64,
    pub quantity: i32,
    pub category_id: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateItem {
    pub name: String,
    pub price: f64,
    pub quantity: i32,
    pub category_id: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateItem {
    pub name: Option<String>,
    pub price: Option<f64>,
    pub quantity: Option<i32>,
    pub category_id: Option<i64>,
}

#[derive(Debug, sqlx::FromRow)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub password_hash: String,
    pub role_id: i64,
}

#[derive(Debug, sqlx::FromRow)]
pub struct Role {
    pub id: i64,
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct RegisterUser {
    pub username: String,
    pub password: String,
    pub role: String,
}

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub id: i64,
    pub username: String,
    pub role_id: i64,
}

#[derive(Debug, Deserialize)]
pub struct ItemQuery {
    pub name: Option<String>,
    pub page: Option<u32>,
    pub page_size: Option<u32>,
}
