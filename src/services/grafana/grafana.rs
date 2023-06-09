use super::models::GrafanaWebhook;
use axum::{response::IntoResponse, Extension, Json};
use reqwest::StatusCode;
use std::sync::Arc;
use webhookntfy::models::New;

pub async fn mygrafana(
    Extension(config): Extension<Arc<New>>,
    Json(payload): Json<GrafanaWebhook>,
) -> impl IntoResponse {
    println!("test");
    for i in payload.alerts {
        let title: String = config
            .service
            .title
            .to_owned()
            .unwrap_or(i.labels.alertname);
        let message = config.service.message.clone().unwrap_or_else(|| {
            i.annotations
                .summary
                .clone()
                .unwrap_or_else(|| String::from("alert"))
        });

        let action = if let Some(action) = config.service.button {
            if action {
                Some(format!(
                    "{}{}{}{}",
                    "view, Silence, ",
                    i.silence_url,
                    ", clear=false; view, See details, ",
                    i.panel_url
                ))
            } else {
                None
            }
        } else {
            None
        };

        webhookntfy::ntfy::ntfy(
            axum::extract::State(config.user.to_owned().into()),
            title.to_owned(),
            message.to_owned(),
            config.service.to_owned(),
            action,
        )
        .await;
    }

    StatusCode::OK
}
