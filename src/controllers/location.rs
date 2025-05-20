use std::fmt::Debug;
use axum::extract::State;
use axum::Json;
use mongodm::bson::Bson;
use crate::dtos::location_dto::LocationDataDTO;
use crate::services::location_service::LocationService;

#[derive(Debug, Clone)]
pub struct LocationController {
    service: LocationService
}

impl LocationController {
    pub fn new(service: LocationService) -> Self {
        Self {
            service
        }
    }
}

pub async fn save_location(
    State(controller): State<LocationController>,
    Json(dto): Json<LocationDataDTO>
) -> Result<Json<Bson>, String> {
    controller.service.save_location(dto)
        .await
        .map(Json)
}
