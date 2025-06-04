use actix_web::{App, HttpServer, middleware::Logger};
use sqlx::SqlitePool;
use xchat_clone::{db, crypto::Crypto, routes};
use std::fs;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    fs::create_dir_all("/data/uploads").unwrap(); // Create persistent uploads directory
    let pool = db::init_db().await;
    let crypto = Crypto::new();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(crypto.clone()))
            .wrap(Logger::default())
            .route("/register", web::post().to(routes::register_user))
            .route("/message", web::post().to(routes::send_message))
            .route("/messages/{user_id}", web::get().to(routes::get_user_messages))
            .route("/upload", web::post().to(routes::upload_file))
            .route("/signal", web::post().to(routes::signal))
            .route("/", web::get().to(routes::serve_index))
            .service(actix_files::Files::new("/static", "./static").show_files_listing())
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
