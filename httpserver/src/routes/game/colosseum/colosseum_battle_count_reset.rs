use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::ColosseumBattleCountResetRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::colosseum::colosseum_battle_count_reset;
use sqlx::SqlitePool;

#[put("ColosseumBattleCountReset")]
async fn colosseum_battle_count_reset_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<ColosseumBattleCountResetRequest>("ColosseumBattleCountReset", &body).map_err(|e| {
        tracing::warn!("Failed to parse ColosseumBattleCountReset: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = colosseum_battle_count_reset::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
