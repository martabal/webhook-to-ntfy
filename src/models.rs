
use serde::{Deserialize, Serialize};

use crate::userinfo::Userinfos;

#[derive(Debug, Serialize, Deserialize,Clone)]
pub struct Servicesconfig{
    pub services : Vec<Services>,

}
#[derive(Debug, Serialize, Deserialize,Clone)]
pub struct Services {

    pub name : String,
    pub config : Config
    
}
#[derive(Debug, Serialize, Deserialize,Clone)]
pub struct Config {
    pub icon: Option<String>,
    pub priority: Option<i32>,
    pub delay: Option<String>,
    pub topic: String,
    pub message: Option<String>,
    pub title: Option<String>,
}

pub struct New {
    pub servicee : Config,
    pub userinfoo : Userinfos,
}










