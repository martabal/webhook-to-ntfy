use std::sync::Arc;
use axum::{Json, response::IntoResponse, Extension};
use reqwest::StatusCode;
use webhookntfy::{models::New};
use super::models::OverseerrWebhook;

pub async fn myoverseerr(Extension(myuser): Extension<Arc<New>>, Json(payload): Json<OverseerrWebhook>) -> impl IntoResponse {

    let title : String = myuser.servicee.title.to_owned().unwrap_or( payload.subject);
    let message : String = myuser.servicee.message.to_owned().unwrap_or(payload.message);


    webhookntfy::ntfy::ntfy(axum::extract::State(myuser.userinfoo.to_owned().into()), title, message, myuser.servicee.to_owned()).await;

    
    StatusCode::OK
    
}