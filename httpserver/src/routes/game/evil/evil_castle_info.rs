use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::EvilCastleInfoRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::evil::evil_castle_info;
use sqlx::SqlitePool;

#[put("EvilCastleInfo")]
async fn evil_castle_info_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<EvilCastleInfoRequest>("EvilCastleInfo", &body).map_err(|e| {
        tracing::warn!("Failed to parse EvilCastleInfo: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = evil_castle_info::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
