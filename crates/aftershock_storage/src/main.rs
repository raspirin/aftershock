use std::env;

use aftershock_storage::{
    create_router,
    migration::run_migrations,
};
use dotenvy::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let port = env::var("AFTERSHOCK_DB_PORT").expect("AFTERSHOCK_DB_PORT is expected");
    let addr = format!("0.0.0.0:{port}");
    run_migrations().expect("Fail to run migrations");

    let app = create_router();

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
