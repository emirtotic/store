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