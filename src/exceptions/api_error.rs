use axum::http::{StatusCode};
use axum::Json;
use axum::response::{IntoResponse, Response};

#[derive(Debug)]
pub struct ApiError(anyhow::Error);

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let error_message = self.0.to_string();
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({
                "error": error_message
            }))
        ).into_response()
    }

}

impl From<anyhow::Error> for ApiError {
    fn from(err: anyhow::Error) -> Self {
        ApiError(err)
    }
}
