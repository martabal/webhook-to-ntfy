use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Serialize, Deserialize, Debug)]
pub struct OverseerrWebhook {
    pub notification_type: String,
    pub subject: String,
    pub message: String,
    pub image: String,
    pub email: String,
    pub username: String,
    pub avatar: String,
}
