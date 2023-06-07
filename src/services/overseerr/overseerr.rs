use super::models::OverseerrWebhook;
use axum::{response::IntoResponse, Extension, Json};
use reqwest::StatusCode;
use std::sync::Arc;
use webhookntfy::models::New;

pub async fn myoverseerr(
    Extension(myuser): Extension<Arc<New>>,
    Json(payload): Json<OverseerrWebhook>,
) -> impl IntoResponse {
    let title: String = myuser.servicee.title.to_owned().unwrap_or(payload.subject);
    let message: String = myuser
        .servicee
        .message
        .to_owned()
        .unwrap_or(payload.message);

    webhookntfy::ntfy::ntfy(
        axum::extract::State(myuser.userinfoo.to_owned().into()),
        title,
        message,
        myuser.servicee.to_owned(),
        None,
    )
    .await;

    StatusCode::OK
}
