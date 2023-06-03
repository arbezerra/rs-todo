use axum::{extract::Extension, routing::{get, post, put, delete}, Router};
use dotenv::dotenv;
use sqlx::mysql::MySqlPoolOptions;

use std::net::SocketAddr;

mod errors;
mod controllers;
mod models;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set.");

    let conn = MySqlPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
        .expect("could not connect to database_url");

    let app = Router::new()
        .route("/", get(index))
        .route("/tasks", get(controllers::task::find_all))
        .route("/task", post(controllers::task::create_task))
        .route("/task/:id", get(controllers::task::find_task))
        .route("/task/:id", put(controllers::task::update_task))
        .route("/task/:id", delete(controllers::task::delete_task))
        .layer(Extension(conn));

    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn index() -> &'static str {
    "OK"
}
