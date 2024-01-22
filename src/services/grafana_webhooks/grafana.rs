use super::models::GrafanaWebhook;
use axum::{http::StatusCode, Extension, Json};

use std::sync::Arc;
use webhookntfy::models::New;

pub async fn mygrafana(
    Extension(config): Extension<Arc<New>>,
    Json(payload): Json<GrafanaWebhook>,
) -> StatusCode {
    for i in payload.alerts {
        let title = config
            .service
            .title
            .clone()
            .unwrap_or_else(|| i.labels.alertname.clone());

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
            axum::extract::State(config.user.clone().into()),
            title.clone(),
            message.clone(),
            config.service.clone(),
            action,
        )
        .await;
    }

    StatusCode::OK
}
