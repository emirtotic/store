use axum::{
    routing::{get, post, delete},
    Router,
};
use sqlx::MySqlPool;
use crate::auth::{login_user, register_user};
use crate::handlers::*;
use axum::middleware;
use crate::auth_middleware::{require_auth, require_role};

pub fn create_routes(pool: MySqlPool) -> Router {
    let public_routes = Router::new()
        .route("/auth/register", post(register_user))
        .route("/auth/login", post(login_user));

    let open_routes = Router::new()
        .route("/items", get(get_all_items))
        .route("/items/:id", get(get_item))
        .route("/items/category/:id", get(get_items_by_category))
        .route("/categories", get(get_all_categories))
        .route("/categories/:id", get(get_category_by_id))
        .route("/items/search", get(search_items))
        .route("/items/search/category/:category_name", get(get_items_by_category_name));

    let protected_routes = Router::new()
        .route("/items/create", post(create_item))
        .route("/items/:id", post(update_item))
        .route("/items/:id", delete(delete_item))
        .layer(middleware::from_fn(require_role("seller")));

    public_routes
        .merge(
            open_routes
                .merge(protected_routes)
                .layer(middleware::from_fn(require_auth)),
        )
        .with_state(pool)
}
