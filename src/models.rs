use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::{DateTime, Utc};

#[derive(FromRow, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub public_key: String,
}

#[derive(FromRow, Serialize, Deserialize)]
pub struct Message {
    pub id: i32,
    pub sender_id: i32,
    pub recipient_id: i32,
    pub content: String,
    pub iv: String,
    pub expires_at: Option<DateTime<Utc>>,
}

#[derive(FromRow, Serialize, Deserialize)]
pub struct File {
    pub id: i32,
    pub message_id: i32,
    pub filename: String,
    pub file_path: String,
}
