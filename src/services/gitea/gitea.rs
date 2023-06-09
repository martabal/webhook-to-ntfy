use super::models::GiteaWebhook;
use axum::{response::IntoResponse, Extension, Json};
use reqwest::StatusCode;
use std::sync::Arc;
use webhookntfy::models::New;

pub async fn mygitea(
    Extension(config): Extension<Arc<New>>,
    Json(payload): Json<GiteaWebhook>,
) -> impl IntoResponse {
    for i in payload.commits {
        let title: String = config.service.title.to_owned().unwrap_or(
            "New commit from ".to_owned() + &i.author.name + " on " + &payload.repository.name,
        );
        let message: String = config.service.message.to_owned().unwrap_or(i.message);
        webhookntfy::ntfy::ntfy(
            axum::extract::State(config.user.to_owned().into()),
            title,
            message,
            config.service.to_owned(),
            None,
        )
        .await;
    }

    StatusCode::OK
}
