use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use crate::{AppError, AppState, CreateAgent, UpdateAgent};

/// List all agent in a chat
#[utoipa::path(
    get,
    path = "/api/chats/{chat_id}/agents",
    params(
        ("chat_id" = u64, Path, description = "Chat id")
    ),
    responses(
        (status = 200, description = "List of agents", body = Vec<ChatAgent>)
    ),
    security(
        ("token" = [])
    )
)]
pub(crate) async fn list_agent_handler(
    Path(chat_id): Path<u64>,
    State(state): State<AppState>,
) -> Result<impl IntoResponse, AppError> {
    let agents = state.list_agents(chat_id).await?;
    Ok((StatusCode::OK, Json(agents)))
}

/// Create agent in a chat
#[utoipa::path(
    post,
    path = "/api/chats/{chat_id}/agents",
    params(
        ("chat_id" = u64, Path, description = "Chat id")
    ),
    responses(
        (status = 200, description = "Agent created", body = ChatAgent)
    ),
    security(
        ("token" = [])
    )
)]
pub(crate) async fn create_agent_handler(
    Path(chat_id): Path<u64>,
    State(state): State<AppState>,
    Json(input): Json<CreateAgent>,
) -> Result<impl IntoResponse, AppError> {
    let agent = state.create_agent(input, chat_id).await?;
    Ok((StatusCode::CREATED, Json(agent)))
}

/// Update agent in a chat
#[utoipa::path(
    patch,
    path = "/api/chats/{chat_id}/agents",
    params(
        ("chat_id" = u64, Path, description = "Chat id")
    ),
    responses(
        (status = 201, description = "Agent updated", body = ChatAgent)
    ),
    security(
        ("token" = [])
    )
)]
pub(crate) async fn update_agent_handler(
    Path(chat_id): Path<u64>,
    State(state): State<AppState>,
    Json(input): Json<UpdateAgent>,
) -> Result<impl IntoResponse, AppError> {
    let agent = state.update_agent(input, chat_id).await?;
    Ok((StatusCode::OK, Json(agent)))
}
