use std::fmt::Debug;
use axum::extract::State;
use axum::Json;
use mongodm::bson::Bson;
use crate::dtos::location_dto::LocationDataDTO;
use crate::dtos::wrapper::ApiResponse;
use crate::exceptions::api_error::ApiError;
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
) -> Result<ApiResponse<Bson>, ApiResponse<()>> {
    tracing::info!("Saving location: {:?}", &dto);
    controller.service.save_location(dto).await
        .map(|e| ApiResponse::success(Some(e)))
        .map_err(|e| e.into())
}
