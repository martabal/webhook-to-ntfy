use super::models::OverseerrWebhook;
use axum::{response::IntoResponse, Extension, Json};
use reqwest::StatusCode;
use std::sync::Arc;
use webhookntfy::models::New;

pub async fn myoverseerr(
    Extension(config): Extension<Arc<New>>,
    Json(payload): Json<OverseerrWebhook>,
) -> impl IntoResponse {
    let title: String = config.service.title.to_owned().unwrap_or(payload.subject);
    let message: String = config.service.message.to_owned().unwrap_or(payload.message);

    webhookntfy::ntfy::ntfy(
        axum::extract::State(config.user.to_owned().into()),
        title,
        message,
        config.service.to_owned(),
        None,
    )
    .await;

    StatusCode::OK
}
