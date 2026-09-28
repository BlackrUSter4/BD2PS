use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::ColosseumBattleEndRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::colosseum::colosseum_battle_end;
use sqlx::SqlitePool;

#[put("ColosseumBattleEnd")]
async fn colosseum_battle_end_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<ColosseumBattleEndRequest>("ColosseumBattleEnd", &body).map_err(|e| {
        tracing::warn!("Failed to parse ColosseumBattleEnd: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = colosseum_battle_end::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
