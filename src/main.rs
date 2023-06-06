use axum::{
    routing::{get, post},
    Extension, Router,
};
use dotenv::dotenv;
use serde_json::{json, Value};
use serde_yaml;
use services::{gitea, grafana, overseerr};
use std::sync::Arc;
use std::{env, net::SocketAddr};
use webhookntfy::{
    models::{New, Servicesconfig},
    userinfo::Userinfos,
};
mod services;

#[tokio::main]
async fn main() {
    let myinit = init();

    let scrape_config: Servicesconfig = inityml();

    let mut routes: Router = Router::new().route("/healthcheck", get(healthcheck));

    for (i, _) in scrape_config.services.iter().enumerate() {
        if scrape_config.services[i].name.to_lowercase() == "gitea" {
            println!("Gitea activated");
            routes = routes
                .route("/gitea", post(gitea::gitea::mygitea))
                .layer(Extension(Arc::new(New {
                    servicee: scrape_config.services[i].config.to_owned(),
                    userinfoo: myinit.to_owned(),
                })));
        }
        if scrape_config.services[i].name.to_lowercase() == "grafana" {
            println!("Grafana activated");
            routes = routes
                .route("/grafana", post(grafana::grafana::mygrafana))
                .layer(Extension(Arc::new(New {
                    servicee: scrape_config.services[i].config.to_owned(),
                    userinfoo: myinit.to_owned(),
                })));
        }
        if scrape_config.services[i].name.to_lowercase() == "overseerr" {
            println!("Overseerr activated");
            routes = routes
                .route("/overseerr", post(overseerr::overseerr::myoverseerr))
                .layer(Extension(Arc::new(New {
                    servicee: scrape_config.services[i].config.to_owned(),
                    userinfoo: myinit.to_owned(),
                })));
        }
    }

    let port = std::env::var("PORT")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(3000);
    let address = SocketAddr::from(([0, 0, 0, 0], port));

    axum::Server::bind(&address)
        .serve(routes.into_make_service())
        .await
        .unwrap();
}

async fn healthcheck() -> axum::extract::Json<Value> {
    json!({"status": 200}).into()
}

fn inityml() -> Servicesconfig {
    match env::var("MODE") {
        Ok(_) => {
            let f = std::fs::File::open("/config/config.yaml").expect("Could not open file.");
            let scrape_config: Servicesconfig =
                serde_yaml::from_reader(f).expect("Could not read values.");
            return scrape_config;
        }
        Err(_) => {
            let f = std::fs::File::open("config.yaml").expect("Could not open file.");
            let scrape_config: Servicesconfig =
                serde_yaml::from_reader(f).expect("Could not read values.");
            return scrape_config;
        }
    }
}

fn init() -> webhookntfy::userinfo::Userinfos {
    let args: Vec<String> = env::args().collect();
    let mut myuser = Userinfos::default();
    if args.iter().any(|i| i == "-e") {
        dotenv().ok();
        println!("Using .env file");
    } else {
        println!("Using environment variables");
    }

    match env::var("NTFY_BASE_URL") {
        Ok(val) => myuser.ntfybaseurl = val,
        Err(_) => panic!("NTFY_BASE_URL must be set."),
    }
    match env::var("NTFY_USERNAME") {
        Ok(val) => myuser.username = val,
        Err(_) => panic!("NTFY_BASE_URL must be set."),
    }
    match env::var("NTFY_PASSWORD") {
        Ok(val) => myuser.password = val,
        Err(_) => panic!("NTFY_BASE_URL must be set."),
    }

    const VERSION: &str = env!("CARGO_PKG_VERSION");
    const AUTHOR: &str = env!("CARGO_PKG_AUTHORS");
    println!("Author : {}", AUTHOR);
    println!("webhookntfy version : {}", VERSION);
    println!("Ntfy base URL : {}", myuser.ntfybaseurl);
    println!("Username : {}", myuser.username);
    println!("Password : {}", maskpassword(myuser.password.to_owned()));
    println!("Started");
    return myuser;
}

fn maskpassword(t: String) -> String {
    let mut mask = String::new();
    for _ in t.chars() {
        mask.push('*');
    }
    mask
}
