use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::IbDungeonGiveUpRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::ib::ib_dungeon_give_up;
use sqlx::SqlitePool;

#[put("IbDungeonGiveUp")]
async fn ib_dungeon_give_up_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<IbDungeonGiveUpRequest>("IbDungeonGiveUp", &body).map_err(|e| {
        tracing::warn!("Failed to parse IbDungeonGiveUp: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = ib_dungeon_give_up::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
