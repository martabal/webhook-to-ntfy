use axum::{http::Request, middleware::Next, response::Response, extract::State};
use base64::{engine::general_purpose, Engine};
use serde_json::{Value, json};

use crate::models::User;

fn token_is_valid(request: String, user: String) -> Result<(), ()> {
    let encoded: String = general_purpose::STANDARD.encode(user);
    if encoded == request {
        Ok(())
    } else {
        Err(())
    }
}

    pub async fn auth<B>(
        State(myuser): State<User>,
        req: Request<B>,
        next: Next<B>,
    ) -> Result<Response, (axum::http::StatusCode, axum::extract::Json<Value>)> {
        let authheader = req
            .headers()
            .get(axum::http::header::AUTHORIZATION)
            .and_then(|header| header.to_str().ok());

        match authheader {
            Some(myheader) => match token_is_valid(myheader.strip_prefix("Basic ").unwrap().to_string(), format!("{}:{}",myuser.username, myuser.password) ) {
                Ok(_) => {

                    Ok(next.run(req).await)
                }
                Err(_) => {
                    return Err((
                        axum::http::StatusCode::UNAUTHORIZED,
                        json!({"error_message":"Wrong authentication","success":false}).into(),
                    ))
                }
            },
            _ => {
                return Err((
                    axum::http::StatusCode::BAD_REQUEST,
                    json!({"error_message":"Your authentication header is missing","success":false}).into(),
                ))
            }
        }
    }