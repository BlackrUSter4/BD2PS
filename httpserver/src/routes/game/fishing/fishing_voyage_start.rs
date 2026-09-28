use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::FishingVoyageStartRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::fishing::fishing_voyage_start;
use sqlx::SqlitePool;

#[put("FishingVoyageStart")]
async fn fishing_voyage_start_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<FishingVoyageStartRequest>("FishingVoyageStart", &body).map_err(|e| {
        tracing::warn!("Failed to parse FishingVoyageStart: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = fishing_voyage_start::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
