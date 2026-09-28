use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::PvpBattleHistoryRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::pvp::pvp_battle_history;
use sqlx::SqlitePool;

#[put("PvpBattleHistory")]
async fn pvp_battle_history_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<PvpBattleHistoryRequest>("PvpBattleHistory", &body).map_err(|e| {
        tracing::warn!("Failed to parse PvpBattleHistory: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = pvp_battle_history::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
