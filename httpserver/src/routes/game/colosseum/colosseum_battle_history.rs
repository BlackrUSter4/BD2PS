use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::ColosseumBattleHistoryRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::colosseum::colosseum_battle_history;
use sqlx::SqlitePool;

#[put("ColosseumBattleHistory")]
async fn colosseum_battle_history_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<ColosseumBattleHistoryRequest>("ColosseumBattleHistory", &body).map_err(|e| {
        tracing::warn!("Failed to parse ColosseumBattleHistory: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = colosseum_battle_history::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
