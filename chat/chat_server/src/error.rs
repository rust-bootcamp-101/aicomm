use axum::{extract::multipart, http::StatusCode, response::IntoResponse, Json};
use chat_core::AgentError;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use utoipa::ToSchema;

#[derive(Debug, ToSchema, Serialize, Deserialize)]
pub struct ErrorOutput {
    pub error: String,
}

#[derive(Debug, Error)]
pub enum AppError {
    #[error("sql error: {0}")]
    SqlxError(#[from] sqlx::Error),

    #[error("password hash error: {0}")]
    PasswordHashError(#[from] argon2::password_hash::Error),

    #[error("general error: {0}")]
    AnyError(#[from] anyhow::Error),

    #[error("http header parse error: {0}")]
    HttpHeaderError(#[from] axum::http::header::InvalidHeaderValue),

    #[error("email already exists: {0}")]
    EmailAlreadyExists(String),

    #[error("create chat error: {0}")]
    CreateChatError(String),

    #[error("not found: {0}")]
    NotFound(String),

    #[error("io error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("multipart error: {0}")]
    MultipartError(#[from] multipart::MultipartError),

    #[error("create content error: {0}")]
    CreateMessageError(String),

    #[error("{0}")]
    ChatFileError(String),

    #[error("{0}")]
    UnAuthorization(String),

    #[error("user {user_id} is not member of chat {chat_id}")]
    NotChatMemberError { user_id: u64, chat_id: u64 },

    #[error("create agent error: {0}")]
    CreateAgentError(String),

    #[error("update agent error: {0}")]
    UpdateAgentError(String),

    #[error("ai agent error: {0}")]
    AiAgentError(#[from] AgentError),
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
            Self::PasswordHashError(_) | Self::HttpHeaderError(_) | Self::MultipartError(_) => {
                StatusCode::UNPROCESSABLE_ENTITY
            }
            Self::SqlxError(_) | Self::AnyError(_) | Self::IoError(_) | Self::AiAgentError(_) => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
            Self::EmailAlreadyExists(_) => StatusCode::CONFLICT,
            Self::CreateChatError(_)
            | Self::CreateMessageError(_)
            | Self::ChatFileError(_)
            | Self::CreateAgentError(_)
            | Self::UpdateAgentError(_) => StatusCode::BAD_REQUEST,
            Self::NotFound(_) => StatusCode::NOT_FOUND,
            Self::UnAuthorization(_) => StatusCode::UNAUTHORIZED,
            Self::NotChatMemberError { .. } => StatusCode::FORBIDDEN,
        };

        (status, Json(ErrorOutput::new(self.to_string()))).into_response()
    }
}
