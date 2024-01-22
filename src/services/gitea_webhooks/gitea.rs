use super::models::GiteaWebhook;
use axum::{http::StatusCode, Extension, Json};
use std::sync::Arc;
use webhookntfy::models::New;

pub async fn mygitea(
    Extension(config): Extension<Arc<New>>,
    Json(payload): Json<GiteaWebhook>,
) -> StatusCode {
    for i in payload.commits {
        let title: String = config.service.title.clone().unwrap_or(
            "New commit from ".to_owned() + &i.author.name + " on " + &payload.repository.name,
        );
        let message: String = config.service.message.clone().unwrap_or(i.message);
        webhookntfy::ntfy::ntfy(
            axum::extract::State(config.user.clone().into()),
            title,
            message,
            config.service.clone(),
            None,
        )
        .await;
    }

    StatusCode::OK
}
