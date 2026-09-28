use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::IbDungeonEnterRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::ib::ib_dungeon_enter;
use sqlx::SqlitePool;

#[put("IbDungeonEnter")]
async fn ib_dungeon_enter_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<IbDungeonEnterRequest>("IbDungeonEnter", &body).map_err(|e| {
        tracing::warn!("Failed to parse IbDungeonEnter: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = ib_dungeon_enter::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
