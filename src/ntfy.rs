
use std::{sync::Arc};
use axum::extract::State;
use reqwest;
use crate::{userinfo::{Userinfos}, models::Config};

pub async fn ntfy(State(myuser): State<Arc<Userinfos>>, title : String, message : String, myconfig : Config)  {
    let url = format!("{}/{}", &myuser.ntfybaseurl,myconfig.topic);
    let client = reqwest::Client::new();

    let mypriority = getpriority(myconfig.priority);
    let mut resp = client
    .post(&url)
    .basic_auth(&myuser.username, Some(&myuser.password))
    .header("Title", title)
    .header("Priority",mypriority)
    .body(message);

    match &myconfig.icon{
        Some(x) =>{
            resp = resp.header("Tags", x);
        }
        None => {
            
        }
    }
    
    match resp.send().await {
        Ok(_) => {println!("Notifications sent to {}", &url)}
        Err(_) => println!("{} is not accessible !", &url),
    }
}


fn getpriority(mytest : Option<i32>) -> String{
    match mytest{
        Some(x) =>{
            return x.to_string();
        }
        None => {
            return "3".to_owned(); // Default value
        }
    }
}

