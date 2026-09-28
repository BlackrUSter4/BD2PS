use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::PvpBattleStartRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::pvp::pvp_battle_start;
use sqlx::SqlitePool;

#[put("PvpBattleStart")]
async fn pvp_battle_start_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<PvpBattleStartRequest>("PvpBattleStart", &body).map_err(|e| {
        tracing::warn!("Failed to parse PvpBattleStart: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = pvp_battle_start::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
