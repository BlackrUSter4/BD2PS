use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::MonsterHuntQuickBattleRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::monster::monster_hunt_quick_battle;
use sqlx::SqlitePool;

#[put("MonsterHuntQuickBattle")]
async fn monster_hunt_quick_battle_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<MonsterHuntQuickBattleRequest>("MonsterHuntQuickBattle", &body)
        .map_err(|e| {
            tracing::warn!("Failed to parse MonsterHuntQuickBattle: {}", e);
            actix_web::error::ErrorBadRequest("Invalid packet")
        })?;
    let response = monster_hunt_quick_battle::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
