use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::BattleStartRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::battle::battle_start;
use sqlx::SqlitePool;

#[put("BattleStart")]
async fn battle_start_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<BattleStartRequest>("BattleStart", &body).map_err(|e| {
        tracing::warn!("Failed to parse BattleStart: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = battle_start::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
