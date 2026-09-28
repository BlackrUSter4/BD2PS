use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::ColosseumBattleStartRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::colosseum::colosseum_battle_start;
use sqlx::SqlitePool;

#[put("ColosseumBattleStart")]
async fn colosseum_battle_start_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<ColosseumBattleStartRequest>("ColosseumBattleStart", &body).map_err(|e| {
        tracing::warn!("Failed to parse ColosseumBattleStart: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = colosseum_battle_start::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
