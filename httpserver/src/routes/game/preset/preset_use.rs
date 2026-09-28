use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::PresetUseRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::preset::preset_use;
use sqlx::SqlitePool;

#[put("PresetUse")]
async fn preset_use_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<PresetUseRequest>("PresetUse", &body).map_err(|e| {
        tracing::warn!("Failed to parse PresetUse: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = preset_use::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
