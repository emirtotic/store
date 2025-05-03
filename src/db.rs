use sqlx::{mysql::MySqlPoolOptions, MySql, Pool};
use std::env;

pub async fn init_db_pool() -> Pool<MySql> {
    let full_url = env::var("DATABASE_URL").expect("DATABASE_URL is missing.");

    let mut parts = full_url.split('@');
    let creds = parts.next().unwrap();
    let host_and_db = parts.next().unwrap();

    let mut host_parts = host_and_db.split('/');
    let host = host_parts.next().unwrap();
    let db_name = host_parts.next().unwrap();

    let base_url = format!("{}@{}", creds, host);

    let server_pool = MySqlPoolOptions::new()
        .max_connections(2)
        .connect(&base_url)
        .await
        .expect("MySQL connection failed.");

    let query = format!("CREATE DATABASE IF NOT EXISTS `{}`", db_name);

    sqlx::query(&query)
        .execute(&server_pool)
        .await
        .expect("Creating database failed.");

    MySqlPoolOptions::new()
        .max_connections(10)
        .connect(&full_url)
        .await
        .expect("MYSQL connection failed.")
}
