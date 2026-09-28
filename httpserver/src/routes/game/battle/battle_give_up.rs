use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::BattleGiveUpRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::battle::battle_give_up;
use sqlx::SqlitePool;

#[put("BattleGiveUp")]
async fn battle_give_up_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<BattleGiveUpRequest>("BattleGiveUp", &body).map_err(|e| {
        tracing::warn!("Failed to parse BattleGiveUp: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = battle_give_up::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
