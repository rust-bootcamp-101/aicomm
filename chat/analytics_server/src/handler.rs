use axum::{
    extract::State,
    http::{request::Parts, StatusCode},
    response::IntoResponse,
};
use tracing::info;

use crate::{
    events::AnalyticsEventRow,
    extractors::{Geo, Protobuf},
    pb::AnalyticsEvent,
    AppError, AppState,
};

/// Create a event
#[utoipa::path(
    post,
    path = "/api/event",
    responses(
        (status = 201, description = "Event created"),
        (status = 400, description = "Invalid event", body = ErrorOutput),
        (status = 500, description = "Internal server error", body = ErrorOutput)
    ),
    security(
        ("token" = [])
    )
)]
pub(crate) async fn create_event_handler(
    parts: Parts,
    State(state): State<AppState>,
    Geo(geo): Geo,
    Protobuf(event): Protobuf<AnalyticsEvent>,
) -> Result<impl IntoResponse, AppError> {
    info!("received event: {:?}", event);
    let mut row = AnalyticsEventRow::try_from(event)?;
    row.update_with_server_info(&parts, geo);

    let mut insert = state.client.insert("analytics_events")?;
    insert.write(&row).await?;
    insert.end().await?;
    Ok(StatusCode::CREATED)
}
