use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::FishingBaitUseRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::fishing::fishing_bait_use;
use sqlx::SqlitePool;

#[put("FishingBaitUse")]
async fn fishing_bait_use_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<FishingBaitUseRequest>("FishingBaitUse", &body).map_err(|e| {
        tracing::warn!("Failed to parse FishingBaitUse: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = fishing_bait_use::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
