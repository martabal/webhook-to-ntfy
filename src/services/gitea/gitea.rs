use super::models::GiteaWebhook;
use axum::{response::IntoResponse, Extension, Json};
use reqwest::StatusCode;
use std::sync::Arc;
use webhookntfy::models::New;

pub async fn mygitea(
    Extension(myuser): Extension<Arc<New>>,
    Json(payload): Json<GiteaWebhook>,
) -> impl IntoResponse {
    for i in payload.commits {
        let title: String = myuser.servicee.title.to_owned().unwrap_or(
            "New commit from ".to_owned() + &i.author.name + " on " + &payload.repository.name,
        );
        let message: String = myuser.servicee.message.to_owned().unwrap_or(i.message);
        webhookntfy::ntfy::ntfy(
            axum::extract::State(myuser.userinfoo.to_owned().into()),
            title,
            message,
            myuser.servicee.to_owned(),
            None,
        )
        .await;
    }

    StatusCode::OK
}
