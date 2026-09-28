use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::BattleVerifyStateRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::battle::battle_verify_state;
use sqlx::SqlitePool;

#[put("BattleVerifyState")]
async fn battle_verify_state_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req =
        parse_packet::<BattleVerifyStateRequest>("BattleVerifyState", &body).map_err(|e| {
            tracing::warn!("Failed to parse BattleVerifyState: {}", e);
            actix_web::error::ErrorBadRequest("Invalid packet")
        })?;
    let response = battle_verify_state::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
