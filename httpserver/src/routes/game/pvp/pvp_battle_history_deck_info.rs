use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::PvpBattleHistoryDeckInfoRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::pvp::pvp_battle_history_deck_info;
use sqlx::SqlitePool;

#[put("PvpBattleHistoryDeckInfo")]
async fn pvp_battle_history_deck_info_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<PvpBattleHistoryDeckInfoRequest>("PvpBattleHistoryDeckInfo", &body)
        .map_err(|e| {
            tracing::warn!("Failed to parse PvpBattleHistoryDeckInfo: {}", e);
            actix_web::error::ErrorBadRequest("Invalid packet")
        })?;
    let response = pvp_battle_history_deck_info::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
