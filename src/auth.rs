use axum::{extract::Request, extract::State, middleware::Next, response::Response};
use base64::{engine::general_purpose, Engine};
use serde_json::{json, Value};

use crate::models::User;

fn token_is_valid(request: &str, user: String) -> Result<(), ()> {
    let encoded: String = general_purpose::STANDARD.encode(user);
    if encoded == request {
        Ok(())
    } else {
        Err(())
    }
}

#[allow(clippy::missing_errors_doc)]
#[allow(clippy::missing_panics_doc)]
pub async fn auth(
    State(myuser): State<User>,
    req: Request,
    next: Next,
) -> Result<Response, (axum::http::StatusCode, axum::extract::Json<Value>)> {
    let authheader = req
        .headers()
        .get(axum::http::header::AUTHORIZATION)
        .and_then(|header| header.to_str().ok());

    match authheader {
        Some(myheader) => match token_is_valid(
            myheader.strip_prefix("Basic ").unwrap(),
            format!("{}:{}", myuser.username, myuser.password),
        ) {
            Ok(()) => Ok(next.run(req).await),
            Err(()) => Err((
                axum::http::StatusCode::UNAUTHORIZED,
                json!({"error_message":"Wrong authentication","success":false}).into(),
            )),
        },
        _ => Err((
            axum::http::StatusCode::BAD_REQUEST,
            json!({"error_message":"Your authentication header is missing","success":false}).into(),
        )),
    }
}
