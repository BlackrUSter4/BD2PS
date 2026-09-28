use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::FishingVoyageEndRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::fishing::fishing_voyage_end;
use sqlx::SqlitePool;

#[put("FishingVoyageEnd")]
async fn fishing_voyage_end_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<FishingVoyageEndRequest>("FishingVoyageEnd", &body).map_err(|e| {
        tracing::warn!("Failed to parse FishingVoyageEnd: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = fishing_voyage_end::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
