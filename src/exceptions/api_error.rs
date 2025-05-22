use crate::dtos::wrapper::ApiResponse;
use axum::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use mongodm::mongo::error::Error;
use mongodm::prelude::MongoError;

#[derive(Debug)]
pub struct ApiError(anyhow::Error);

impl From<anyhow::Error> for ApiError {
    fn from(err: anyhow::Error) -> Self {
        ApiError(err)
    }
}

impl From<MongoError> for ApiError {
    fn from(value: MongoError) -> Self {
        ApiError(value.into())
    }
}

impl From<ApiError> for ApiResponse<()> {
    fn from(value: ApiError) -> Self {
        ApiResponse {
            success: false,
            data: None,
            error: Some(value.0.to_string()),
        }
    }
}
