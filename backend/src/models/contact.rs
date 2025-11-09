use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Deserialize)]
pub struct ContactForm {
    pub name: String,
    pub phone: String,
    pub email: String,
    pub company: String,
    pub service: String,
    pub message: String,
}

#[derive(Serialize, Deserialize)]
pub struct ContactMessage {
    pub name: String,
    pub email: String,
    pub phone: String,
    pub company: String,
    pub service: String,
    pub message: String,
    pub created_at: DateTime<Utc>,
}