mod controller;
mod db;
mod model;
mod repository;
mod service;

use dotenv::dotenv;
use std::env;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let app_router: axum::Router = controller::create_app_router();

    let addr: String = env::var("APP_URL").expect("APP_URL must be set");

    println!("Listening on {}", addr);

    axum::Server::bind(&addr.parse().unwrap())
        .serve(app_router.into_make_service())
        .await
        .unwrap();
}
