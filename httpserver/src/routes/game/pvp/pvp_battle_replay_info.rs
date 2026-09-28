use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::PvpBattleReplayInfoRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::pvp::pvp_battle_replay_info;
use sqlx::SqlitePool;

#[put("PvpBattleReplayInfo")]
async fn pvp_battle_replay_info_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req =
        parse_packet::<PvpBattleReplayInfoRequest>("PvpBattleReplayInfo", &body).map_err(|e| {
            tracing::warn!("Failed to parse PvpBattleReplayInfo: {}", e);
            actix_web::error::ErrorBadRequest("Invalid packet")
        })?;
    let response = pvp_battle_replay_info::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
