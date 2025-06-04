use actix_web::{web, App, HttpServer, middleware::Logger};
use std::fs;

// Declare modules
mod db;
mod crypto;
mod routes;
mod models;
mod webrtc;
mod utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    if let Err(e) = fs::create_dir_all("/data/uploads") {
        eprintln!("Failed to create /data/uploads: {}. Continuing without directory creation.", e);
    }
    let pool = db::init_db().await;
    let crypto = crypto::Crypto::new();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(crypto.clone()))
            .wrap(Logger::default())
            .route("/register", web::post().to(routes::register_user))
            .route("/message", web::post().to(routes::send_message))
            .route("/messages/{user_id}", web::get().to(routes::get_user_messages))
            .route("/upload", web::post().to(routes::upload_file))
            .route("/signal", web::post().to(webrtc::signal))
            .route("/", web::get().to(routes::serve_index))
            .service(actix_files::Files::new("/static", "./static").show_files_listing())
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}