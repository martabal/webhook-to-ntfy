use std::sync::Arc;
use axum::{Json, response::IntoResponse, Extension};
use reqwest::StatusCode;
use webhookntfy::{models::New};
use super::models::GrafanaWebhook;

pub async fn mygrafana(Extension(myuser): Extension<Arc<New>>, Json(payload): Json<GrafanaWebhook>) -> impl IntoResponse {
    for i in payload.alerts {
        let title : String = myuser.servicee.title.to_owned().unwrap_or( i.labels.alertname);
        let message : String = myuser.servicee.message.to_owned().unwrap_or(i.annotations.description);
        webhookntfy::ntfy::ntfy(axum::extract::State(myuser.userinfoo.to_owned().into()), title, message, myuser.servicee.to_owned()).await;
    }
    
    StatusCode::OK
    
}