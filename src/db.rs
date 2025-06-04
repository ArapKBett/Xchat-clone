use sqlx::{SqlitePool, FromRow};
use chrono::{DateTime, Utc};
use crate::models::{User, Message, File};

pub async fn init_db() -> SqlitePool {
    let pool = SqlitePool::connect("sqlite:///data/xchat.db").await.unwrap();
    sqlx::migrate!("./migrations").run(&pool).await.unwrap();
    pool
}

pub async fn create_user(pool: &SqlitePool, username: &str, public_key: &str) -> Result<User, sqlx::Error> {
    let user = sqlx::query_as::<_, User>(
        "INSERT INTO users (username, public_key) VALUES (?, ?) RETURNING *"
    )
    .bind(username)
    .bind(public_key)
    .fetch_one(pool)
    .await?;
    Ok(user)
}

pub async fn get_user_by_id(pool: &SqlitePool, user_id: i32) -> Result<User, sqlx::Error> {
    let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = ?")
        .bind(user_id)
        .fetch_one(pool)
        .await?;
    Ok(user)
}

pub async fn save_message(
    pool: &SqlitePool,
    sender_id: i32,
    recipient_id: i32,
    content: &str,
    iv: &str,
    expires_at: Option<DateTime<Utc>>,
) -> Result<Message, sqlx::Error> {
    let message = sqlx::query_as::<_, Message>(
        "INSERT INTO messages (sender_id, recipient_id, content, iv, expires_at) VALUES (?, ?, ?, ?, ?) RETURNING *"
    )
    .bind(sender_id)
    .bind(recipient_id)
    .bind(content)
    .bind(iv)
    .bind(expires_at)
    .fetch_one(pool)
    .await?;
    Ok(message)
}

pub async fn get_messages(pool: &SqlitePool, user_id: i32) -> Result<Vec<Message>, sqlx::Error> {
    let messages = sqlx::query_as::<_, Message>(
        "SELECT * FROM messages WHERE recipient_id = ? AND (expires_at IS NULL OR expires_at > ?)"
    )
    .bind(user_id)
    .bind(Utc::now())
    .fetch_all(pool)
    .await?;
    Ok(messages)
}

pub async fn save_file(
    pool: &SqlitePool,
    message_id: i32,
    filename: &str,
    file_path: &str,
) -> Result<File, sqlx::Error> {
    let file = sqlx::query_as::<_, File>(
        "INSERT INTO files (message_id, filename, file_path) VALUES (?, ?, ?) RETURNING *"
    )
    .bind(message_id)
    .bind(filename)
    .bind(file_path)
    .fetch_one(pool)
    .await?;
    Ok(file)
    }
