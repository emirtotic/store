use axum::{
    routing::{get, post, delete},
    Router,
};
use sqlx::MySqlPool;

use crate::handlers::*;

pub fn create_routes(pool: MySqlPool) -> Router {
    Router::new()
        .route("/items", get(get_all_items))
        .route("/items/:id", get(get_item))
        .route("/items/create", post(create_item))
        .route("/items/:id", post(update_item))
        .route("/items/:id", delete(delete_item))
        .route("/items/category/:id", get(get_items_by_category))
        .route("/categories", get(get_all_categories))
        .route("/categories/:id", get(get_category_by_id))
        .with_state(pool)
}
