use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::PvpBattleRankUserDetailRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::pvp::pvp_battle_rank_user_detail;
use sqlx::SqlitePool;

#[put("PvpBattleRankUserDetail")]
async fn pvp_battle_rank_user_detail_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<PvpBattleRankUserDetailRequest>("PvpBattleRankUserDetail", &body)
        .map_err(|e| {
            tracing::warn!("Failed to parse PvpBattleRankUserDetail: {}", e);
            actix_web::error::ErrorBadRequest("Invalid packet")
        })?;
    let response = pvp_battle_rank_user_detail::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
