use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::PvpBattleResetRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::pvp::pvp_battle_reset;
use sqlx::SqlitePool;

#[put("PvpBattleReset")]
async fn pvp_battle_reset_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<PvpBattleResetRequest>("PvpBattleReset", &body).map_err(|e| {
        tracing::warn!("Failed to parse PvpBattleReset: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = pvp_battle_reset::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
