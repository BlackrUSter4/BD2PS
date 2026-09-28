use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::BattleRetryRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::battle::battle_retry;
use sqlx::SqlitePool;

#[put("BattleRetry")]
async fn battle_retry_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<BattleRetryRequest>("BattleRetry", &body).map_err(|e| {
        tracing::warn!("Failed to parse BattleRetry: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = battle_retry::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
