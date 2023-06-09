use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Servicesconfig {
    pub services: Vec<Services>,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Services {
    pub name: String,
    pub config: Config,
    pub auth: Option<User>,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub icon: Option<String>,
    pub priority: Option<i32>,
    pub delay: Option<String>,
    pub topic: String,
    pub message: Option<String>,
    pub title: Option<String>,
    pub button: Option<bool>,
}

pub struct New {
    pub service: Config,
    pub user: Userinfos,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Action {
    pub action: String,
    pub label: String,
    pub url: String,
    pub clear: bool,
}

#[derive(Debug, Default, Clone)]
pub struct Userinfos {
    pub ntfybaseurl: String,
    pub username: String,
    pub password: String,
}
