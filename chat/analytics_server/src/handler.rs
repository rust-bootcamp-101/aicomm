use axum::{
    extract::State,
    http::{request::Parts, StatusCode},
    response::IntoResponse,
};
use chat_core::User;

use crate::{
    events::AnalyticsEventRow, extractors::Protobuf, pb::AnalyticsEvent, AppError, AppState,
};

/// Create a event
#[utoipa::path(
    post,
    path = "/api/event",
    responses(
        (status = 201, description = "Event created"),
        (status = 400, description = "Invalid event", body = ErrorOutput)
    ),
    security(
        ("token" = [])
    )
)]
pub(crate) async fn create_event_handler(
    parts: Parts,
    State(state): State<AppState>,
    Protobuf(event): Protobuf<AnalyticsEvent>,
) -> Result<impl IntoResponse, AppError> {
    let mut row = AnalyticsEventRow::try_from(event)?;
    // get user info from extension
    if let Some(user) = parts.extensions.get::<User>() {
        row.user_id = Some(user.id.to_string());
    } else {
        row.user_id = None;
    }
    let mut insert = state.client.insert("analytics_events")?;
    insert.write(&row).await?;
    insert.end().await?;
    Ok(StatusCode::CREATED)
}
