use crate::models::{CreateItem, Item, Category};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde_json::json;
use sqlx::MySqlPool;

pub async fn get_all_items(State(pool): State<MySqlPool>) -> Result<Json<Vec<Item>>, StatusCode> {
    tracing::info!("Retrieving all items from database...");

    let items = sqlx::query_as!(
        Item,
        r#"SELECT id, name, price, quantity, category_id FROM items"#
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| {
        tracing::error!("Database error: {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Json(items))
}

pub async fn get_item(
    Path(id): Path<i64>,
    State(pool): State<MySqlPool>,
) -> Result<Json<Item>, StatusCode> {
    tracing::info!("GET /items/{}", id);

    let item = sqlx::query_as!(
        Item,
        r#"
        SELECT id, name, price, quantity, category_id
        FROM items
        WHERE id = ?
        "#,
        id
    )
    .fetch_optional(&pool)
    .await
    .map_err(|e| {
        tracing::error!("DB error: {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    match item {
        Some(i) => Ok(Json(i)),
        None => {
            tracing::error!("Item with id {} is not found", id);
            Err(StatusCode::NOT_FOUND)
        }
    }
}

pub async fn update_item(
    Path(id): Path<i64>,
    State(pool): State<MySqlPool>,
    Json(payload): Json<CreateItem>,
) -> Result<Json<Item>, StatusCode> {
    tracing::info!("POST /items/{}", id);

    let existing = sqlx::query_as!(
        Item,
        r#"
        SELECT id, name, price, quantity, category_id
        FROM items
        WHERE id = ?
        "#,
        id
    )
        .fetch_optional(&pool)
        .await
        .map_err(|e| {
            tracing::error!("DB error: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    let _existing = match existing {
        Some(item) => item,
        None => {
            tracing::warn!("Item {} not found for update", id);
            return Err(StatusCode::NOT_FOUND);
        }
    };

    if payload.name.trim().len() < 3 {
        tracing::warn!("Invalid name: {}", payload.name);
        return Err(StatusCode::BAD_REQUEST);
    }

    if payload.price < 0.0 {
        tracing::warn!("Invalid price: {}", payload.price);
        return Err(StatusCode::BAD_REQUEST);
    }

    if payload.quantity < 1 {
        tracing::warn!("Invalid quantity: {}", payload.quantity);
        return Err(StatusCode::BAD_REQUEST);
    }

    if let Some(cat_id) = payload.category_id {
        let category_exists = sqlx::query_scalar!(
        r#"
        SELECT EXISTS(
            SELECT 1 FROM categories WHERE id = ?
        ) AS exists_flag
        "#,
        cat_id
    )
            .fetch_one(&pool)
            .await
            .map_err(|e| {
                tracing::error!("Failed to check category existence: {:?}", e);
                StatusCode::INTERNAL_SERVER_ERROR
            })?;

        if category_exists == 0 {
            tracing::warn!("Category with ID {} does not exist", cat_id);
            return Err(StatusCode::BAD_REQUEST);
        }
    }

    let name = payload.name;
    let price = payload.price;
    let quantity = payload.quantity;
    let category_id = payload.category_id;

    sqlx::query!(
        r#"
        UPDATE items
        SET name = ?, price = ?, quantity = ?, category_id = ?
        WHERE id = ?
        "#,
        name,
        price,
        quantity,
        category_id,
        id
    )
        .execute(&pool)
        .await
        .map_err(|e| {
            tracing::error!("Update failed: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    let updated = sqlx::query_as!(
        Item,
        r#"
        SELECT id, name, price, quantity, category_id
        FROM items
        WHERE id = ?
        "#,
        id
    )
        .fetch_one(&pool)
        .await
        .map_err(|e| {
            tracing::error!("Failed to fetch updated item: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(Json(updated))
}

pub async fn delete_item(
    Path(id): Path<i64>,
    State(pool): State<MySqlPool>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    tracing::info!("DELETE /items/{}", id);

    let existing = sqlx::query_as!(
        Item,
        r#"
        SELECT id, name, price, quantity, category_id
        FROM items
        WHERE id = ?
        "#,
        id
    )
        .fetch_optional(&pool)
        .await
        .map_err(|e| {
            tracing::error!("DB error: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    let _existing = match existing {
        Some(item) => item,
        None => {
            tracing::warn!("Item {} not found for update", id);
            return Err(StatusCode::NOT_FOUND);
        }
    };

    sqlx::query!(
        r#"
        DELETE FROM items WHERE id = ?
        "#,
        id
    )
        .execute(&pool)
        .await
        .map_err(|e| {
            tracing::error!("Delete failed: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(Json(json!({ "message": "Item has been removed." })))
}

pub async fn get_items_by_category(
    Path(id): Path<i64>,
    State(pool): State<MySqlPool>,
) -> Result<Json<Vec<Item>>, StatusCode> {
    tracing::info!("GET /items/category/{}", id);

    let items = sqlx::query_as!(
        Item,
        r#"
    SELECT id, name, price, quantity, category_id
    FROM items
    WHERE category_id = ?
    "#,
        id
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| {
        tracing::error!("Database error: {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Json(items))
}

pub async fn create_item(
    State(pool): State<MySqlPool>,
    Json(payload): Json<CreateItem>,
) -> Result<Json<Item>, StatusCode> {
    tracing::info!("POST /items/create: {:?}", payload);

    if payload.name.trim().len() < 3 {
        tracing::warn!("Invalid name: {}", payload.name);
        return Err(StatusCode::BAD_REQUEST);
    }

    if payload.price < 0.0 {
        tracing::warn!("Invalid price: {}", payload.price);
        return Err(StatusCode::BAD_REQUEST);
    }

    if payload.quantity < 1 {
        tracing::warn!("Invalid quantity: {}", payload.quantity);
        return Err(StatusCode::BAD_REQUEST);
    }

    if let Some(cat_id) = payload.category_id {
        let category_exists = sqlx::query_scalar!(
        r#"
        SELECT EXISTS(
            SELECT 1 FROM categories WHERE id = ?
        ) AS exists_flag
        "#,
        cat_id
    )
            .fetch_one(&pool)
            .await
            .map_err(|e| {
                tracing::error!("Failed to check category existence: {:?}", e);
                StatusCode::INTERNAL_SERVER_ERROR
            })?;

        if category_exists == 0 {
            tracing::warn!("Category with ID {} does not exist", cat_id);
            return Err(StatusCode::BAD_REQUEST);
        }
    }

    let result = sqlx::query!(
        r#"
        INSERT INTO items (name, price, quantity, category_id)
        VALUES (?, ?, ?, ?)
        "#,
        payload.name,
        payload.price,
        payload.quantity,
        payload.category_id
    )
        .execute(&pool)
        .await
        .map_err(|e| {
            tracing::error!("Insert failed: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    let inserted_id = result.last_insert_id() as i64;

    let item = sqlx::query_as!(
        Item,
        r#"
        SELECT id, name, price, quantity, category_id
        FROM items
        WHERE id = ?
        "#,
        inserted_id
    )
        .fetch_one(&pool)
        .await
        .map_err(|e| {
            tracing::error!("Fetch inserted item failed: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(Json(item))
}


// CATEGORIES

pub async fn get_all_categories(
    State(pool): State<MySqlPool>,
) -> Result<Json<Vec<Category>>, StatusCode> {
    tracing::info!("GET /categories");

    let categories = sqlx::query_as!(
        Category,
        r#"
        SELECT id, name FROM categories
        "#
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to fetch categories: {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Json(categories))
}

pub async fn get_category_by_id(
    Path(id): Path<i64>,
    State(pool): State<MySqlPool>,
) -> Result<Json<Category>, StatusCode> {
    tracing::info!("GET /categories/{}", id);

    let category = sqlx::query_as!(
        Category,
        r#"
        SELECT id, name
        FROM categories
        WHERE id = ?
        "#,
        id
    )
    .fetch_optional(&pool)
    .await
    .map_err(|e| {
        tracing::error!("DB error: {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    match category {
        Some(c) => Ok(Json(c)),
        None => {
            tracing::error!("Category with id {} is not found", id);
            Err(StatusCode::NOT_FOUND)
        }
    }
}
