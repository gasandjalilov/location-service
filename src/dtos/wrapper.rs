use axum::Json;
use axum::response::IntoResponse;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ApiResponse<T: Serialize> {
    pub success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

impl<T: Serialize> ApiResponse<T> {
    pub fn success(data: Option<T>) -> Self {
        Self {
            success: true,
            data,
            error: None,
        }
    }

    pub fn error(error: String) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(error),
        }
    }
}

impl<T: Serialize> IntoResponse for ApiResponse<T> {
    fn into_response(self) -> axum::response::Response {
        Json(self).into_response()
    }
}
