use actix_web::{web, HttpResponse, Responder};
use actix_files::NamedFile;
use actix_multipart::Multipart;
use sqlx::SqlitePool;
use serde_json::Value;
use chrono::{Duration, Utc};
use crate::{db::{create_user, save_message, get_messages, save_file}, crypto::Crypto, utils, models::Message};

pub async fn register_user(
    pool: web::Data<SqlitePool>,
    crypto: web::Data<Crypto>,
    data: web::Json<Value>,
) -> impl Responder {
    let username = data["username"].as_str().unwrap_or_default();
    let (secret_key, public_key) = crypto.generate_keypair();
    match create_user(pool.as_ref(), username, &public_key.to_string()).await {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(_) => HttpResponse::BadRequest().body("Username already exists"),
    }
}

pub async fn send_message(
    pool: web::Data<SqlitePool>,
    crypto: web::Data<Crypto>,
    data: web::Json<Value>,
) -> impl Responder {
    let sender_id = data["sender_id"].as_i64().unwrap_or_default() as i32;
    let recipient_id = data["recipient_id"].as_i64().unwrap_or_default() as i32;
    let content = data["content"].as_str().unwrap_or_default();
    let expires_in = data["expires_in"].as_i64().map(|secs| Utc::now() + Duration::seconds(secs));

    // Simulate shared secret (replace with proper ECDH)
    let shared_secret = [0u8; 32];
    let (ciphertext, iv) = crypto.encrypt_message(content, &shared_secret);
    match save_message(pool.as_ref(), sender_id, recipient_id, &ciphertext, &iv, expires_in).await {
        Ok(message) => HttpResponse::Ok().json(message),
        Err(_) => HttpResponse::InternalServerError().body("Failed to save message"),
    }
}

pub async fn get_user_messages(
    pool: web::Data<SqlitePool>,
    crypto: web::Data<Crypto>,
    path: web::Path<i32>,
) -> impl Responder {
    let user_id = path.into_inner();
    match get_messages(pool.as_ref(), user_id).await {
        Ok(messages) => {
            // Decrypt messages (simplified; assumes shared secret)
            let shared_secret = [0u8; 32];
            let decrypted: Vec<_> = messages
                .into_iter()
                .map(|m| {
                    let plaintext = crypto.decrypt_message(&m.content, &m.iv, &shared_secret);
                    Message { content: plaintext, ..m }
                })
                .collect();
            HttpResponse::Ok().json(decrypted)
        }
        Err(_) => HttpResponse::InternalServerError().body("Failed to fetch messages"),
    }
}

pub async fn upload_file(
    pool: web::Data<SqlitePool>,
    payload: Multipart,
    data: web::Query<Value>,
) -> impl Responder {
    let message_id = data["message_id"].as_i64().unwrap_or_default() as i32;
    match utils::save_file(payload).await {
        Ok(file_path) => {
            let filename = file_path.split('/').last().unwrap_or("unknown");
            match save_file(pool.as_ref(), message_id, filename, &file_path).await {
                Ok(file) => HttpResponse::Ok().json(file),
                Err(_) => HttpResponse::InternalServerError().body("Failed to save file"),
            }
        }
        Err(_) => HttpResponse::InternalServerError().body("Failed to upload file"),
    }
}

pub async fn serve_index() -> impl Responder {
    NamedFile::open("static/index.html")
}
