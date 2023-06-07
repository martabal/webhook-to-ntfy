use crate::{models::Config, userinfo::Userinfos};
use axum::extract::State;
use reqwest;
use std::sync::Arc;

pub async fn ntfy(
    State(myuser): State<Arc<Userinfos>>,
    title: String,
    message: String,
    myconfig: Config,
    action: Option<String>,
) {
    let url = format!("{}/{}", &myuser.ntfybaseurl, myconfig.topic);
    let client = reqwest::Client::new();

    let mypriority = getpriority(myconfig.priority);
    let mut resp = client
        .post(&url)
        .basic_auth(&myuser.username, Some(&myuser.password))
        .header("Title", title)
        .header("Priority", mypriority)
        .body(message);

    match &myconfig.icon {
        Some(x) => {
            resp = resp.header("Tags", x);
        }
        None => {}
    }
    match action {
        Some(x) => resp = resp.header("Actions", x),
        None => {}
    }

    match resp.send().await {
        Ok(_) => {
            println!("Notifications sent to {}", &url)
        }
        Err(_) => println!("{} is not accessible !", &url),
    }
}

fn getpriority(priority: Option<i32>) -> String {
    match priority {
        Some(x) => {
            return x.to_string();
        }
        None => {
            return "3".to_owned(); // Default value
        }
    }
}
