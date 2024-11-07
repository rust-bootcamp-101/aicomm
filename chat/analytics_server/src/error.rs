use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use tracing::warn;
use utoipa::ToSchema;

#[derive(Debug, ToSchema, Serialize, Deserialize)]
pub struct ErrorOutput {
    pub error: String,
}

#[derive(Debug, Error)]
pub enum AppError {
    #[error("clickhouse error: {0}")]
    ClickhouseError(#[from] clickhouse::error::Error),

    #[error("missing event context")]
    MissingEventContext,

    #[error("missing event data")]
    MissingEventData,

    #[error("missing system info")]
    MissingSystemInfo,

    #[error("general error: {0}")]
    AnyError(#[from] anyhow::Error),
}

impl ErrorOutput {
    pub fn new(error: impl Into<String>) -> Self {
        Self {
            error: error.into(),
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let status = match &self {
            Self::ClickhouseError(_) | Self::AnyError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::MissingEventContext | Self::MissingEventData | Self::MissingSystemInfo => {
                StatusCode::BAD_REQUEST
            }
        };
        let msg = self.to_string();
        warn!("Status {}, error: {}", status, msg);

        (status, Json(ErrorOutput::new(self.to_string()))).into_response()
    }
}
