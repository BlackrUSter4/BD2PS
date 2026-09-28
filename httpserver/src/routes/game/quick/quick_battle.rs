use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::QuickBattleRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::quick::quick_battle;
use sqlx::SqlitePool;

#[put("QuickBattle")]
async fn quick_battle_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<QuickBattleRequest>("QuickBattle", &body).map_err(|e| {
        tracing::warn!("Failed to parse QuickBattle: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = quick_battle::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
