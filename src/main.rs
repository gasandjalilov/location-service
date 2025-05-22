mod config;
mod controllers;
mod dtos;
mod exceptions;
mod models;
mod repositories;
mod services;

use crate::config::configs::DatabaseConfig;
use crate::controllers::location::{LocationController, save_location};
use crate::repositories::location_repository::LocationRepository;
use crate::services::location_service::LocationService;
use axum::Router;
use axum::routing::post;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let cofig = DatabaseConfig::from_env();
    let db = config::database::MNGDatabase::new(&cofig)
        .await
        .expect("Failed to connect to database");

    let repo = LocationRepository::new(db.db);
    let service = LocationService::new(repo);
    let controller = LocationController::new(service);

    let app = Router::new()
        .route("/location", post(save_location))
        .with_state(controller);

    let listener = tokio::net::TcpListener::bind(
        env::var("BASE_URL").unwrap_or_else(|_| "0.0.0.0:8080".to_string()),
    )
    .await
    .expect("Failed to bind to port");

    axum::serve(listener, app)
        .await
        .expect("Error while serving");

    Ok(())
}
