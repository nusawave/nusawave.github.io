use serde::{Deserialize, Serialize};
use sqlx::FromRow;
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

#[derive(FromRow, Serialize)]
pub struct ContactMessage {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub phone: String,
    pub company: String,
    pub service: String,
    pub message: String,
    pub created_at: Option<DateTime<Utc>>, // <- pakai Option di sini
}