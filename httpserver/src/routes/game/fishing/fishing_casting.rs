use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::FishingCastingRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::fishing::fishing_casting;
use sqlx::SqlitePool;

#[put("FishingCasting")]
async fn fishing_casting_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<FishingCastingRequest>("FishingCasting", &body).map_err(|e| {
        tracing::warn!("Failed to parse FishingCasting: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = fishing_casting::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
