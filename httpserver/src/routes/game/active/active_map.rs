use actix_web::{HttpResponse, Result, put, web};
use bd2::proto::proto_net::ActiveMapRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::active::active_map;
use sqlx::SqlitePool;

#[put("ActiveMap")]
async fn active_map_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<ActiveMapRequest>("ActiveMap", &body).map_err(|e| {
        tracing::warn!("Failed to parse ActiveMap: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = active_map::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
