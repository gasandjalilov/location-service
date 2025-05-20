mod controllers;
mod models;
mod repositories;
mod services;
mod dtos;
mod config;
mod exceptions;

use axum::Router;
use axum::routing::{post};
use crate::config::configs::DatabaseConfig;
use crate::controllers::location::{save_location, LocationController};
use crate::repositories::location_repository::LocationRepository;
use crate::services::location_service::LocationService;

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

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("Failed to bind to port");
    
    axum::serve(listener, app)
        .await
        .expect("Error while serving");
    
    Ok(())
}
