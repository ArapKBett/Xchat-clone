use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct SignalData {
    sdp: String,
    candidate: Option<String>,
}

pub async fn signal(data: web::Json<SignalData>) -> HttpResponse {
    // In a real app, broadcast SDP/candidate to the other peer via WebSocket
    HttpResponse::Ok().json(data.0)
}
