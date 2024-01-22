use super::models::OverseerrWebhook;
use axum::{http::StatusCode, Extension, Json};

use std::sync::Arc;
use webhookntfy::models::New;

pub async fn myoverseerr(
    Extension(config): Extension<Arc<New>>,
    Json(payload): Json<OverseerrWebhook>,
) -> StatusCode {
    let title: String = config.service.title.clone().unwrap_or(payload.subject);
    let message: String = config.service.message.clone().unwrap_or(payload.message);

    webhookntfy::ntfy::ntfy(
        axum::extract::State(config.user.clone().into()),
        title,
        message,
        config.service.clone(),
        None,
    )
    .await;

    StatusCode::OK
}
