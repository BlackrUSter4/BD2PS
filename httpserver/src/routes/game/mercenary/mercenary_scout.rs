use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::MercenaryScoutRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::mercenary::mercenary_scout;
use sqlx::SqlitePool;

#[put("MercenaryScout")]
async fn mercenary_scout_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<MercenaryScoutRequest>("MercenaryScout", &body).map_err(|e| {
        tracing::warn!("Failed to parse MercenaryScout: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = mercenary_scout::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
