use super::models::GrafanaWebhook;
use axum::{response::IntoResponse, Extension, Json};
use reqwest::StatusCode;
use std::sync::Arc;
use webhookntfy::models::New;

pub async fn mygrafana(
    Extension(myuser): Extension<Arc<New>>,
    Json(payload): Json<GrafanaWebhook>,
) -> impl IntoResponse {
    println!("test");
    for i in payload.alerts {
        let title: String = myuser
            .servicee
            .title
            .to_owned()
            .unwrap_or(i.labels.alertname);
        let message: String = match &myuser.servicee.message {
            Some(msg) => msg.to_owned(),
            None => match &i.annotations.summary {
                Some(summary) => summary.to_owned(),
                None => String::new(), // Provide a default value here if needed
            },
        };

        let action: String = format!(
            "{}{}{}{}",
            "view, Silence, ", i.silence_url, ", clear=false; view, See details, ", i.panel_url
        );

        webhookntfy::ntfy::ntfy(
            axum::extract::State(myuser.userinfoo.to_owned().into()),
            title,
            message,
            myuser.servicee.to_owned(),
            Some(action),
        )
        .await;
    }

    StatusCode::OK
}
