use axum::{
    routing::{get, post},
    Extension, Router,
};
use dotenv::dotenv;
use serde_json::{json, Value};

use services::{gitea_webhooks, grafana_webhooks, overseer_webhooks};
use std::sync::Arc;
use std::{env, net::SocketAddr};
use webhookntfy::models::{New, Servicesconfig, Userinfos};

mod services;

#[tokio::main]
async fn main() {
    let myinit = init();

    let scrape_config: Servicesconfig = inityml();

    let mut routes: Router = Router::new().route("/healthcheck", get(healthcheck));

    for service in &scrape_config.services {
        if service.name.to_lowercase() == "gitea" {
            println!("Gitea activated");
            routes = routes.route("/gitea", post(gitea_webhooks::gitea::mygitea));

            if let Some(test) = service.auth.clone() {
                println!("Authentication activated for Grafana");
                routes = routes.layer(axum::middleware::from_fn_with_state(
                    test,
                    webhookntfy::auth::auth,
                ));
            }

            routes = routes.layer(Extension(Arc::new(New {
                service: service.config.clone(),
                user: myinit.clone(),
            })));
        }
        if service.name.to_lowercase() == "grafana" {
            println!("Grafana activated");
            routes = routes.route("/grafana", post(grafana_webhooks::grafana::mygrafana));

            if let Some(test) = service.auth.clone() {
                println!("Authentication activated for Grafana");
                routes = routes.layer(axum::middleware::from_fn_with_state(
                    test,
                    webhookntfy::auth::auth,
                ));
            }

            routes = routes.layer(Extension(Arc::new(New {
                service: service.config.clone(),
                user: myinit.clone(),
            })));
        }
        if service.name.to_lowercase() == "overseerr" {
            println!("Overseerr activated");
            routes = routes.route(
                "/overseerr",
                post(overseer_webhooks::overseerr::myoverseerr),
            );

            if let Some(test) = service.auth.clone() {
                println!("Authentication activated for Overseerr");
                routes = routes.layer(axum::middleware::from_fn_with_state(
                    test,
                    webhookntfy::auth::auth,
                ));
            }

            routes = routes.layer(Extension(Arc::new(New {
                service: service.config.clone(),
                user: myinit.clone(),
            })));
        }
    }

    let port = std::env::var("PORT")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(3000);
    let address = SocketAddr::from(([0, 0, 0, 0], port));

    let listener = tokio::net::TcpListener::bind(address).await.unwrap();

    axum::serve(
        listener,
        routes.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .unwrap();
}

async fn healthcheck() -> axum::extract::Json<Value> {
    json!({"status": 200}).into()
}

fn inityml() -> Servicesconfig {
    let path = if std::path::Path::new("config.yaml").exists() {
        "config.yaml"
    } else {
        "/config/config.yaml"
    };

    let content =
        std::fs::read_to_string(path).unwrap_or_else(|err| panic!("Failed to read {path}: {err}"));

    serde_yaml::from_str(&content).unwrap_or_else(|err| panic!("Failed to parse {path}: {err}"))
}

fn init() -> webhookntfy::models::Userinfos {
    let args: Vec<String> = env::args().collect();
    if args.iter().any(|i| i == "-e") {
        dotenv().ok();
        println!("Using .env file");
    } else {
        println!("Using environment variables");
    }

    let user = Userinfos {
        ntfybaseurl: env::var("NTFY_BASE_URL")
            .unwrap_or_else(|_| panic!("NTFY_BASE_URL must be set.")),
        username: env::var("NTFY_USERNAME")
            .unwrap_or_else(|_| panic!("NTFY_USERNAME must be set.")),
        password: env::var("NTFY_PASSWORD")
            .unwrap_or_else(|_| panic!("NTFY_PASSWORD must be set.")),
    };
    #[allow(clippy::items_after_statements)]
    const VERSION: &str = env!("CARGO_PKG_VERSION");
    #[allow(clippy::items_after_statements)]
    const AUTHOR: &str = env!("CARGO_PKG_AUTHORS");
    println!(
        "Author: {}\n\
        webhookntfy version: {}\n\
        Ntfy base URL: {}\n\
        Username: {}\n\
        Password: {}\n\
        Started",
        AUTHOR,
        VERSION,
        user.ntfybaseurl,
        user.username,
        maskpassword(&user.password)
    );
    user
}

fn maskpassword(t: &str) -> String {
    let mut mask = String::new();
    for _ in t.chars() {
        mask.push('*');
    }
    mask
}

#[cfg(test)]
mod tests {

    use super::*;

    #[tokio::test]
    async fn test_maskpassword() {
        let password = "secretpassword";
        let result = maskpassword(password);
        let supposed = "*".repeat(password.len());
        assert_eq!(supposed, result);
    }
}
