use dotenvy::dotenv;
use std::net::SocketAddr;
use tokio::net::TcpListener;

mod db;
mod handlers;
mod models;
mod routes;

#[tokio::main]
async fn main() {
    dotenv().ok();
    tracing_subscriber::fmt::init();

    let db = db::init_db_pool().await;
    let app = routes::create_routes(db.clone());

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("🚀 Server running at http://{}", addr);

    let listener = TcpListener::bind(addr).await.unwrap();

    axum::serve::serve(listener, app)
        .await
        .unwrap();
}
