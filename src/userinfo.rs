#[derive(Debug, Default,Clone)]
pub struct Userinfos {
    pub ntfybaseurl: String,
    pub topic: String,
    pub username: String,
    pub password: String,
    pub priority: String,
}

#[allow(non_camel_case_types)]
pub enum Priority {
    
    urgent,
    high,
    default,
    low,
    min,
}