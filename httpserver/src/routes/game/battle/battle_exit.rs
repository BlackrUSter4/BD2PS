use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::BattleExitRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::battle::battle_exit;
use sqlx::SqlitePool;

#[put("BattleExit")]
async fn battle_exit_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<BattleExitRequest>("BattleExit", &body).map_err(|e| {
        tracing::warn!("Failed to parse BattleExit: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = battle_exit::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
