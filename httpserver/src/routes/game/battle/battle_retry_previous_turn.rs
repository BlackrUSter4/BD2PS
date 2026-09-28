use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::BattleRetryPreviousTurnRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::battle::battle_retry_previous_turn;
use sqlx::SqlitePool;

#[put("BattleRetryPreviousTurn")]
async fn battle_retry_previous_turn_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<BattleRetryPreviousTurnRequest>("BattleRetryPreviousTurn", &body)
        .map_err(|e| {
            tracing::warn!("Failed to parse BattleRetryPreviousTurn: {}", e);
            actix_web::error::ErrorBadRequest("Invalid packet")
        })?;
    let response = battle_retry_previous_turn::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
