use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::TotalWarBattleEndRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::total::total_war_battle_end;
use sqlx::SqlitePool;

#[put("TotalWarBattleEnd")]
async fn total_war_battle_end_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req =
        parse_packet::<TotalWarBattleEndRequest>("TotalWarBattleEnd", &body).map_err(|e| {
            tracing::warn!("Failed to parse TotalWarBattleEnd: {}", e);
            actix_web::error::ErrorBadRequest("Invalid packet")
        })?;
    let response = total_war_battle_end::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
